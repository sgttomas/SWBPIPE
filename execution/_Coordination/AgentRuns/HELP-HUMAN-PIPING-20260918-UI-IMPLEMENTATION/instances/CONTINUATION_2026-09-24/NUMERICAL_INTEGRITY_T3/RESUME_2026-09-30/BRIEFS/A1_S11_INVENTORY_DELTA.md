# ROOT exact S11 inventory compatibility grant

RV29's unchanged full FK suite found site_table_is_exactly_the_accumulations_of_
the_scanned_files failing at adaptive.rs/run_schedule: table5, source6.
RV29 and ROOT read the actual added site: c+=1 in the new PublicationDecision::
Rejected branch. c is the existing integer schedule index, not a floating sum.

I22 may edit exactly the run_schedule row in FK/tests/s11_site_table.rs from
count5 to6 and annotate the new integer certificate-rejection escalation.
Every other row, scanned file, scanner rule, assertion and floating-accumulation
disposition remains unchanged. This is the only newly authorized maintained
path here. No blanket source-table rebaseline or omission is permitted.
Run the focused s11_site_table integration test once under the existing C host
settings after this exact edit. Retain the original failed full-suite result.
The new count comes from the classified source site, not merely the failure's
observed count. RV29 must backcheck the final diff/result.

This grants no edits to adaptive_tests.rs or kf1_tracker_tests.rs; those still
need the concrete criterion-preserving evidence patch and ROOT review. It grants
no production helper change, broader test retry, old fixture change or new tool.
C's existing time boundary and direct numerical priorities remain.
