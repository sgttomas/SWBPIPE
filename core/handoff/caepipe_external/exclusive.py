"""Create-only writes below one base directory, through verified directory handles (DEL-17-05).

Taken from the sitting pack's `Tree` (branch claude/caepipe-sitting-pack, tools/caepipe_correlation/sitting_pack.py)
into a module of its own, so that the harness and the sitting pack share one layer. Additions here: a name may start
with `_` (the exchange folder's `_harness`), a file this process created can be removed only after re-reading it and
finding exactly the content it wrote (`remove_own`), and files are read without following a link.

Nothing that exists is ever opened for writing: every directory on the way is entered with O_NOFOLLOW | O_DIRECTORY
relative to its parent's handle, a new directory is reserved with `mkdir` (which fails when anything is there), and a
file is created with O_CREAT | O_EXCL | O_NOFOLLOW (which fails when anything is there, a link included). So a hard
link from a stored file to a path written here cannot be written through, and a symbolic link is refused rather than
followed. A file that must appear whole or not at all (a selection) is written complete under a new name of its own and
then given its public name with `link`, which also fails when anything is there. There is no truncating, replacing or
renaming write in this module.
"""

from __future__ import annotations

import errno
import hashlib
import json
import os
from pathlib import Path
import re
import stat
import subprocess
from typing import Any, Iterable, Mapping

# A plain name: letters, digits, `.`, `_` and `-`, not starting with a dot (so never `.` or `..`), no separator.
NAME_PATTERN = re.compile(r"^[A-Za-z0-9_][A-Za-z0-9._-]{0,127}\Z")
_CHUNK = 1 << 20
_DIR_FLAGS = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW
_NEW_FILE_FLAGS = os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW
_READ_FLAGS = os.O_RDONLY | os.O_NOFOLLOW


class ContainmentError(RuntimeError):
    """A path the pilot would write or delete is a symbolic link, lies outside the exchange folder or inside a Git
    work tree, or is otherwise not a place it may write."""


class ExistingPath(ContainmentError):
    """A write would land on a path that already exists; nothing there is replaced."""


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def check_name(name: Any, where: str) -> str:
    if not isinstance(name, str) or not NAME_PATTERN.match(name):
        raise ContainmentError(f"{where}: {name!r} is not a plain name (letters, digits, . _ -; no separator, not "
                               "starting with a dot)")
    return name


def inside_git(path: Path, environ: Mapping[str, str] = os.environ) -> bool | None:
    """True when `path` lies inside a Git work tree or Git directory, False when not, None when git cannot say."""
    env = {key: value for key, value in os.environ.items() if not key.startswith("GIT_")}
    if environ.get("PATH"):
        env["PATH"] = environ["PATH"]
    try:
        completed = subprocess.run(["git", "-C", str(path), "rev-parse", "--is-inside-work-tree"],
                                   capture_output=True, text=True, env=env, check=False, timeout=30)
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return None
    # Exit 0 prints "true" inside a work tree and "false" inside a .git directory: both are refused.
    return completed.returncode == 0


def outside_work_trees(path: Path, what: str) -> Path:
    """`path` made absolute, refused when it (or its nearest existing ancestor) lies inside a Git work tree or Git
    directory."""
    path = Path(os.path.abspath(path))
    probe = path
    while not probe.exists():
        if probe.parent == probe:
            break
        probe = probe.parent
    inside = inside_git(probe)
    if inside is None:
        raise ContainmentError(f"{what}: git is not available to confirm {path} is outside every Git work tree")
    if inside:
        raise ContainmentError(f"{what}: {path} lies inside a Git work tree or Git directory; nothing is written "
                               "inside a repository")
    return path


class Tree:
    """Reads and create-only writes below one base directory, each relative to directory handles.

    The base must lie outside every Git work tree. Every component on the way to a path is opened with O_NOFOLLOW and
    O_DIRECTORY relative to its parent's handle, so a component that is (or becomes) a symbolic link is refused rather
    than followed, and a parent swapped between two operations cannot redirect a write. A directory on the way that
    holds `.git` is refused. A new directory is reserved with `mkdir` (which fails when it exists) and a file is
    created with O_CREAT | O_EXCL | O_NOFOLLOW: an existing path is refused by name and left untouched. Container
    directories (`cases`, `cases/<id>`, `cases/<id>/runs`, ...) are created when absent and otherwise entered under
    the same rules."""

    def __init__(self, base: Path, what: str):
        self.base = outside_work_trees(Path(base), what)
        self.what = what
        try:
            self.fd = os.open(self.base, _DIR_FLAGS)
        except OSError as error:
            raise ContainmentError(f"{what}: {self.base} is not a directory that can be entered without following "
                                   f"a link ({error.strerror})") from None
        self._refuse_git(self.fd, "")

    def close(self) -> None:
        if self.fd >= 0:
            os.close(self.fd)
            self.fd = -1

    def __enter__(self) -> "Tree":
        return self

    def __exit__(self, *exc: Any) -> None:
        self.close()

    def _refuse_git(self, fd: int, where: str) -> None:
        try:
            os.stat(".git", dir_fd=fd, follow_symlinks=False)
        except FileNotFoundError:
            return
        raise ContainmentError(f"{self.what}: {where or '.'} holds .git, a repository or work tree; nothing is "
                               "written inside a Git work tree")

    def _open_dir(self, parts: Iterable[str], *, create: bool) -> int:
        fd = os.dup(self.fd)
        walked: list[str] = []
        try:
            for part in parts:
                check_name(part, f"{self.what} path component")
                walked.append(part)
                try:
                    child = os.open(part, _DIR_FLAGS, dir_fd=fd)
                except FileNotFoundError:
                    if not create:
                        raise
                    try:
                        os.mkdir(part, 0o700, dir_fd=fd)
                    except FileExistsError:
                        pass
                    child = os.open(part, _DIR_FLAGS, dir_fd=fd)
                except OSError as error:
                    if error.errno in (errno.ELOOP, errno.ENOTDIR, errno.EMLINK):
                        raise ContainmentError(f"{self.what}: {'/'.join(walked)} is a symbolic link or not a "
                                               "directory; refused") from None
                    raise
                os.close(fd)
                fd = child
                self._refuse_git(fd, "/".join(walked))
            return fd
        except BaseException:
            os.close(fd)
            raise

    def exists(self, parts: tuple[str, ...]) -> bool:
        try:
            fd = self._open_dir(parts[:-1], create=False)
        except (FileNotFoundError, NotADirectoryError):
            return False
        try:
            os.stat(parts[-1], dir_fd=fd, follow_symlinks=False)
            return True
        except FileNotFoundError:
            return False
        finally:
            os.close(fd)

    def listdir(self, parts: tuple[str, ...]) -> list[str]:
        try:
            fd = self._open_dir(parts, create=False)
        except FileNotFoundError:
            return []
        try:
            return sorted(os.listdir(fd))
        finally:
            os.close(fd)

    def read_bytes(self, parts: tuple[str, ...]) -> bytes | None:
        """A regular file's bytes, never through a link; None when absent or not readable as a regular file."""
        try:
            parent = self._open_dir(parts[:-1], create=False)
        except (FileNotFoundError, NotADirectoryError):
            return None
        try:
            fd = os.open(parts[-1], _READ_FLAGS, dir_fd=parent)
        except OSError:
            return None
        finally:
            os.close(parent)
        try:
            if not stat.S_ISREG(os.fstat(fd).st_mode):
                return None
            data = bytearray()
            while chunk := os.read(fd, _CHUNK):
                data += chunk
            return bytes(data)
        finally:
            os.close(fd)

    def read_json(self, parts: tuple[str, ...]) -> Any:
        """One of the harness's own records, or None when absent or unreadable."""
        data = self.read_bytes(parts)
        if data is None:
            return None
        try:
            return json.loads(data.decode("utf-8"))
        except (UnicodeDecodeError, json.JSONDecodeError):
            return None

    def ensure_dir(self, parts: tuple[str, ...]) -> None:
        """Enter a container directory, creating it when absent; never a link."""
        os.close(self._open_dir(parts, create=True))

    def reserve_dir(self, parts: tuple[str, ...]) -> tuple[str, ...]:
        """Create a new directory; refused by name when anything is already at that path."""
        parent = self._open_dir(parts[:-1], create=True)
        try:
            check_name(parts[-1], f"{self.what} directory")
            os.mkdir(parts[-1], 0o700, dir_fd=parent)
        except FileExistsError:
            raise ExistingPath(f"{self.what}: {'/'.join(parts)} already exists; nothing there is replaced") from None
        finally:
            os.close(parent)
        return parts

    def _new_file(self, parts: tuple[str, ...]) -> int:
        parent = self._open_dir(parts[:-1], create=False)
        try:
            check_name(parts[-1], f"{self.what} file")
            return os.open(parts[-1], _NEW_FILE_FLAGS, 0o600, dir_fd=parent)
        except FileExistsError:
            raise ExistingPath(f"{self.what}: {'/'.join(parts)} already exists; nothing there is replaced") from None
        except OSError as error:
            if error.errno == errno.ELOOP:
                raise ExistingPath(f"{self.what}: {'/'.join(parts)} is a symbolic link; nothing is written through "
                                   "it") from None
            raise
        finally:
            os.close(parent)

    def create(self, parts: tuple[str, ...], data: bytes | str) -> dict[str, Any]:
        """Create a new file holding `data`: {bytes, sha256}."""
        payload = data.encode("utf-8") if isinstance(data, str) else data
        fd = self._new_file(parts)
        try:
            view = memoryview(payload)
            while view:
                view = view[os.write(fd, view):]
            return {"bytes": os.fstat(fd).st_size, "sha256": sha256_bytes(payload)}
        finally:
            os.close(fd)

    def record(self, parts: tuple[str, ...], value: Any) -> dict[str, Any]:
        return self.create(parts, json.dumps(value, indent=2, sort_keys=True, default=str) + "\n")

    def link_new(self, source: tuple[str, ...], target: tuple[str, ...]) -> None:
        """Give the complete file at `source` (one this process created) a second, new name `target`, with `link`,
        which fails when anything is at `target`: the name appears only with the whole content behind it, so a crash
        can leave the source alone but never a partial file at `target`. ExistingPath when `target` exists."""
        source_dir = self._open_dir(source[:-1], create=False)
        try:
            target_dir = self._open_dir(target[:-1], create=True)
            try:
                check_name(target[-1], f"{self.what} file")
                os.link(source[-1], target[-1], src_dir_fd=source_dir, dst_dir_fd=target_dir, follow_symlinks=False)
            except FileExistsError:
                raise ExistingPath(f"{self.what}: {'/'.join(target)} already exists; nothing there is replaced") \
                    from None
            finally:
                os.close(target_dir)
        finally:
            os.close(source_dir)

    def remove_own(self, parts: tuple[str, ...], content: bytes) -> str:
        """Remove a file this process created, only when it is still a regular file holding exactly `content` (and
        the file read is the one unlinked); otherwise leave it. The outcome: `removed` or `left: <why>`."""
        try:
            parent = self._open_dir(parts[:-1], create=False)
        except (FileNotFoundError, NotADirectoryError):
            return "left: its folder is no longer there"
        except ContainmentError as error:
            return f"left: {error}"
        try:
            try:
                fd = os.open(parts[-1], _READ_FLAGS, dir_fd=parent)
            except FileNotFoundError:
                return "left: it is no longer there"
            except OSError as error:
                return f"left: it cannot be opened as a regular file ({error.strerror})"
            try:
                opened = os.fstat(fd)
                if not stat.S_ISREG(opened.st_mode):
                    return "left: it is not a regular file"
                data = bytearray()
                while chunk := os.read(fd, _CHUNK):
                    data += chunk
            finally:
                os.close(fd)
            if bytes(data) != content:
                return "left: its content is not the content this step wrote"
            named = os.stat(parts[-1], dir_fd=parent, follow_symlinks=False)
            if (named.st_dev, named.st_ino) != (opened.st_dev, opened.st_ino):
                return "left: the file at its name is not the one this step read"
            os.unlink(parts[-1], dir_fd=parent)
            return "removed"
        finally:
            os.close(parent)

    def path(self, parts: Iterable[str]) -> Path:
        return self.base.joinpath(*parts)
