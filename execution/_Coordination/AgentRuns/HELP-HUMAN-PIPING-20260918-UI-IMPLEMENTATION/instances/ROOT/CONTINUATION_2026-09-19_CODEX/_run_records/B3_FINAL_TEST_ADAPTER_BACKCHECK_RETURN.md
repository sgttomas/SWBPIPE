**PASS — no actionable finding.**

Verified candidate `7afd31493beee6538d576e69d79777166cf40ff8` against reviewed `bf15aed87571df3601b99c350fc5296fb337cd91`. It is clean and matches manager checkpoint `5045bd1c2c92b4c018e5652111ee6093434ead78` in desktop scope.

Exactly three test lines changed across two files:

- The Shared-section locator now identifies the exact combobox, avoiding its open listbox. The selected section ID and real option-click route remain unchanged.
- Both unit assertions still require exactly `"m"`, now through the compact control’s canonical `data-value`. Every caller of the affected helper uses a migrated node or pipe unit control.

No product bytes, expected outcomes, timeouts, geometry floors, or other desktop files changed. `git diff --check` passes; the sealed brief hash matches.

The six affected journey passes are reported by the manager. I ran no tests or UI and made no writes. Full source/dist completion, clean sweep, and actual-candidate CI remain separate gates; this verdict is not final merge suitability.

Independent TASK/Astra-xhigh attribution and existing telemetry limits apply. Standard F-PIP-2 / DEC-081 claim fence applies.
