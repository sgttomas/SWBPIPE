/** The refusal words of a record whose table binding is not accepted (DEL-05-02; decision (e) of the
 * table-binding unit), written out here rather than read from the implementation, as exact patterns. */
const exact = (text: string) => new RegExp(`^${text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}$`);
export const FORMER_TABLE_REFUSAL_TEXT = "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH: this run was recorded under an earlier revision of these result rules, which this version cannot interpret; the stored run is unchanged; run the analysis again on its saved model state to create a new result";
export const UNRECOGNISED_TABLE_REFUSAL_TEXT = "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH: this run names a revision of these result rules that this version does not recognise; the stored run is unchanged; run the analysis again on its saved model state to create a new result";
export const FORMER_TABLE_REFUSAL = exact(FORMER_TABLE_REFUSAL_TEXT);
export const UNRECOGNISED_TABLE_REFUSAL = exact(UNRECOGNISED_TABLE_REFUSAL_TEXT);
