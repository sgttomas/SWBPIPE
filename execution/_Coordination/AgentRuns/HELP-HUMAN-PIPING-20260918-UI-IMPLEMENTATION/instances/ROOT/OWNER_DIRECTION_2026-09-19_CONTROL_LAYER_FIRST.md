# The owner's direction of 2026-09-19: the control layer first

Recorded by ROOT (HELP_HUMAN, Agent 0). Every text below is extracted from the stored session transcript (stored-transcript custody, not transport bytes). ROOT's own answers are reproduced because the owner's direction refers to them.

## The owner's word to pause

The owner sent this while ROOT was recording the merge of pull request #803; the host queued it and delivered it within the turn. Queued at 2026-09-19T04:11:12.912Z, SHA-256 `54d4598a14535bb7e6ead38ba5892c2a009b06c4e49f542ce812339c3eba61f6`, verbatim:

~~~
Once merged pause and let's take stock of what's transpired and what remains.
~~~

ROOT paused all three threads of work, confirmed each stop, and gave the owner a stock-take (`../../HANDOFF_STATE.md`, the entries of 2026-09-19).

## The owner's first question

2026-09-19T04:20:37.538Z, SHA-256 `56b762258de4fa74f364cb70431416a9500ca7517f751b286b6dcaddc72a61e2`, verbatim:

~~~
Is it fair to say the visual redesign was intended to happen last after the control layer was succinctly developed?
~~~

## ROOT's answer

2026-09-19T04:22:28.385Z, SHA-256 `dbd9411375f7d814a7391fa1e97cc5ca5465e443d7e627faa2c6da3fc1096c78`, verbatim:

~~~
Not as the plan was written. It does match the design's own priorities, though, and in hindsight I think it is the better order.

**What the record says.** Your handoff decision ordered three tranches: the rulings, then "the shell and the canvas over today's operations", then the typed-interface and engine gaps. It did not split the second tranche into a control layer and a visual pass. The plan you approved put design tokens first, then the state extraction, then each surface with its structure and appearance together. The canvas lane ran in parallel from the start, and its first slice was colour. The closest thing to your reading is the handoff's reading order for an implementer. It puts the interface model and the one operation route first, then what changes in meaning, then the gaps, and only then the design system for appearance. I find no instruction that visual work goes last.

**What has actually happened.** The visual thread has run ahead of the control layer.

- **Control layer so far:** one slice, the session extraction in [#803](https://github.com/sgttomas/chirality/pull/803). The handler repairs, the shell, the table through the one operation route, the inspector and the results header have not started.
- **Visual so far:** the tokens slice, the canvas colour slice with its edge line, and the design system amendment. The amendment is in its third round over contrast ratios for controls that mostly do not exist in the product yet. The latest finding is a reading of 2.9985:1 against 3:1.

That sequencing was my call. I put tokens first so both lanes could build from them, and I let the amendment grow through review cycles.

**If you want the control layer first,** everything is paused at a clean point, so the re-cut is cheap.

- **Shell lane.** Proceed as sealed: handler repairs, shell, table, inspector, results, review page. Build each plainly on the tokens already in the product, and judge them by behaviour tests and code review. The design-fidelity review by screenshot moves to the end.
- **Canvas lane.** Selection, isolate, labels and real outside diameter come next. The colour slice and the edge line's look wait. One exception: the edge line's frame-time cost should still be measured early, because D-72 measures orbit with it on.
- **Design system V1.4.** I would do one thing before stopping visual work. Merge what is already reviewed after a backcheck, record the two open contrast items as open, and adopt the new tokens in the product. That gives the control layer the boundary token to reference by name. The contrast questions then wait for a final visual pass, measured on real controls instead of a specimen.

I am still holding everything until you decide.
~~~

## The owner's second question

2026-09-19T04:49:39.610Z, SHA-256 `e0d793dcf5f2c44a3a11b5b09d5adbeea2637eb6b27d891f712c67933d9c5758`, verbatim:

~~~
Describe the differences between the current planned work sequence and the one where the control layer is prioritized over visual change.  What does the model delegation strategy do to influence these differences.  Which way do you recommend and why?
~~~

## ROOT's answer, which is the recommendation the direction refers to

2026-09-19T04:54:04.354Z, SHA-256 `dd1d9d745d1ab787af60c845989fdcee907057b3d77d32b4105c9f0544aaee92`, verbatim:

~~~
I recommend putting the control layer first, with two exceptions where work that looks visual is really structural. The first exception corrects something I told you in my last message.

## How the two sequences differ

| | Plan as written | Control layer first |
|---|---|---|
| Shell lane | Each surface is rebuilt with its behaviour and its final look together. | Same surfaces, same order, built on the tokens already in the product. Layout follows the frames from the start. Colour, contrast, washes and fine spacing wait. |
| Gate per shell slice | Opus code review plus a Fable fidelity review by screenshot. | Opus code review plus behaviour tests. One structure-only fidelity check after the shell slice, and one full fidelity review at the end. |
| Design system amendment | Runs to full closure now. The shell's controls were told to build from its new boundary token. | Lands as reviewed after one backcheck, with its two open contrast items recorded. A small slice adopts the tokens. The open items are settled at the end on real controls. |
| Canvas lane | Colours and edge line, then selection, isolate, labels, real outside diameter. | Almost unchanged. See the exception below. |
| Contrast checks under D-68 | Closed per slice. | Closed once at the end, and each pull request names them as open. Keyboard and focus criteria stay per slice. |
| D-72 qualification | After the canvas lane. | Last, with the edge line's frame cost probed early. |
| Your decisions | Arrive one at a time as visual questions surface. | Arrive as one package at the closing pass. |

**Exception one: the canvas lane finishes its colour and edge-line pull request first.** Last time I said its colour slice and the edge line's look could wait. They should not.

- The colour work is already built on the lane's branch, and the later slices edit the same material and resource code.
- The colours cannot land without the edge line, because the light figure is paler than today's product without it.
- D-72 measures orbit with the line on.

So the lane should finish that first pull request as sealed. After that, its planned order is already mechanism first. Its pure appearance questions are already parked: the halo casing, the silhouette shades, the second edge colour and the deformation view's ink.

**Exception two: layout follows the frames from the start.** Only colour, contrast, washes and fine spacing wait for the closing pass. This is the third column of the shell lane row above.

## What the delegation strategy does

- **It does not block the change.** The lanes are split by file scope, not by layer. Both sequences use the same two managers and the same children. The re-cut costs one sealed addendum per lane.
- **It prices the two kinds of work differently.**
  - Control work passes one independent gate: Opus, a different model from the implementer, checking objective properties. The state extraction passed it in one round.
  - Visual work also passes the fidelity review. Only Fable has the design judgment for that review, so its independence comes from a fresh context alone.
  - In my reading the fidelity review is the costlier gate and the weaker one. The plan as written pays for it on every surface slice. Control first pays for it twice.
- **It amplifies precision loops on visual work.** A Fable author paired with an Opus reviewer who reproduces every ratio finds something smaller each round. Code has a natural stop, because tests pass and the diff freezes. A design specimen has no such stop. The amendment is in its third round, and its latest finding concerns a pressed table row. That table does not exist yet.
- **It sets the usage rate.** Three Fable-led streams with Opus reviewers ran at once, and you hit the session limit twice. Control first runs two streams with one gate each.
- **It decides who can do the late visual work.** On stable structure, the shell's appearance pass is mostly token values and stylesheet rows. That is Sonnet-grade work, and only the judgment calls need Fable or you. Interleaved with structural change, the same work needs Fable throughout.
- **It routes judgment calls to you.** Fable managers and authors hand you appearance questions as open decisions. Most items now open with you are about appearance. None came from the state extraction.

## Why I recommend it

1. **Risk.** The handoff's constraints that protect results are all control-layer: the one operation route, run standing and the generation gates. Only one control slice has landed.
2. **The open questions are in the appearance.** They are better answered once, on real controls, as one package for you.
3. **Cost.** There are fewer streams and one gate per slice. The shell's path also no longer waits on the design system's review loop.
4. **It matches the handoff's reading order.** That order puts appearance after the interface model and the operations map.

It has costs:

- Main looks plain for longer.
- A fidelity problem surfaces later, though the check after the shell slice should limit that to colour and spacing.
- Every surface is touched again at the end, which means one more round of browser-test churn.

One caution on my evidence. The state extraction was a move with no behaviour change, the easiest kind of slice to review. The table and inspector slices will raise more questions than it did. My argument rests on the kind of checking each layer allows, not on that one clean result.

Everything stays paused until you choose.
~~~

## The owner's direction

2026-09-19T05:01:27.493Z, SHA-256 `4fc291f65ae684d82a04fb789b98959a3a8512bae79eb91c5336a7990d4f5ea8`, verbatim:

~~~
I am okay if you first clean up anything that can be tidied quickly and neatly, otherwise prepare the handoff and the prompt according to the revised sequencing you recommend that puts "the control layer first".  Are the Agent 0/1/2 behaviours working as intended?  You told me at one point that all Type 2 instances were communicating to you and you had to distribute the information to the managers.  I'm not sure if that's a failure of the Agent 0/1/2 paradigm or not, because it's meant to be open and flexible.  Are you having the managers spawn their own subagents or are you doing so for them?  Thoughts?
~~~

## ROOT's reading, labelled as ROOT's

- The owner directs that the handoff and the successor's prompt follow "the revised sequencing you recommend that puts 'the control layer first'". ROOT reads that as the owner adopting, for the program from here, the sequence ROOT recommended at 04:54Z, with its two exceptions: the canvas lane finishes its colour and edge-line pull request first, and layout follows the frames from the start. It is recorded as amendment 1 of `../../ORCHESTRATION_PLAN.md`.
- The amendment changes the order of work and the review gates inside Tranche B. It changes no ruling of the owner's (D-68, D-71, D-72, the decision rows), no constraint of the implementation handoff, and no tolerance, oracle or limit. Tranche A2 stays after Tranche B.
- "Clean up anything that can be tidied quickly and neatly": ROOT did the tidying listed in `SESSION_HANDOFF_2026-09-19.md` §2 and started no product work. One thing ROOT had recommended at 04:54Z it did not do: land design system V1.4 after one backcheck. On reading the author's pause note ROOT found that the candidate states two figures that are inexact as it stands, and that making them exact is a full author round and a review, so the amendment is parked with its branch and recorded (decision DS5-D3, `lanes/DESIGN-SYSTEM-05/HANDOFF.md` on branch `codex/swbpipe-design-system-v14-20260918`). This departs from ROOT's own recommendation and is open to the owner.
- The owner's questions about the Agent 0, 1 and 2 behaviours are questions, not directions. ROOT's answer is in the session handoff's §6, with what the record shows and what ROOT would change. The owner said of the paradigm that "it's meant to be open and flexible"; ROOT changes nothing in the role definitions.
- This reading is open to correction.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
