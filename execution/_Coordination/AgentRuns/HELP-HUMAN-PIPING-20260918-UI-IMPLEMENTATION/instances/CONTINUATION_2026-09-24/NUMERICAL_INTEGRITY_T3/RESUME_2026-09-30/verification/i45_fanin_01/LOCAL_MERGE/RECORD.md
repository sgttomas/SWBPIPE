# I45 reviewed local component integration

ROOT merged source 28ac57891cd5786e8f84d28f54aa7f1581f69402 into the numerical branch with a merge commit.
Prior NUM: 6c641f94de522d12f8676074a481eb1224788f22. Local merge: 1d19eb51ba1a31b8507439ec9c191eeac14bdc9f.
Original fresh review a231fb5d0230 found RV60-F1/F2; repair was checked by the
same reviewer at 6c641f94de52, with no remaining actionable finding. ROOT read
both reviews, complete core/test diff and repair, verified all source/packet
hashes, and matched the maintained core/validation trees exactly after the join.
No source conflict or new implementation change occurred during integration.

Original candidate and repaired source-bound evidence, the intentional reversal
of the old invalid-mode unwrap, and actual unchanged W0/W1 refusals are preserved.
This is local conditional component fan-in, not a GitHub/main merge or F2a product
acceptance. Public projection/routing, C2/readers/receipts, source families,
resource/caller qualification, required availability and full gates remain.
The mode-only arbitrary raw-pair guard is still only an internal-caller boundary;
no public raw-identity token or completed C2 custody is asserted.
