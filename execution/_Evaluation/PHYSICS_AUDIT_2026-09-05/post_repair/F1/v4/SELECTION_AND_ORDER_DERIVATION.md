# Fixture selection and pagination derivation

ComparisonPanel renders only first four sorted deltas. Existing first-source-ref/equal-unit matching and descending absolute-delta sort gives: pressure-thrust C150 5400 N; S120 reaction191.444032 N; P120 quarter3 bending-z181.392343 N*m; P120 shear-y168.869577 N; P120 end-j bending-z165.986377 N*m (rank5). Old end-j moment was rank2. Authorized click target becomes same P120 bending-z family at quarter3, preserving Rack span link and N*m context.

Raw generated result order moves base N140 UZ from row34 to51, base magnitude from7 to24. ResultsPanel paginates the input order into50-row pages before grouping. Hence UZ is now on page2, while magnitude remains page1. Existing page2 action later also checks P100 axial, nowrow62. Root notified to move UZ assertion to that existing page2 check, preserving IDs and coverage rather than altering UI.

Base N140 magnitude row is3.977299 mm; both result row readout and computed-knowledge citation reference that same row. Previous3.686623 literals sync to3.977299. This is fixture transport, not independent mechanics verification.
