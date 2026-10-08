#!/bin/sh
# I97 B2-C: collision check of every name the contract introduces (read-only Git; records only).
# Usage: sh b2c_collisions.sh <NUM>   (prints one line per name: name|form|files outside P/execution|first files)
# Each name is searched as a bare fixed string and, for wire values, as the quoted JSON string.
set -u
NUM="$1"
P=projects/chirality-piping
export GIT_OPTIONAL_LOCKS=0
check() {
  form="$1"; name="$2"
  files=$(git -C "$NUM" grep -l -F -e "$name" HEAD -- "$P" ":!$P/execution" | sed "s#^HEAD:$P/##")
  count=$(printf '%s' "$files" | grep -c . || true)
  first=$(printf '%s\n' "$files" | head -3 | tr '\n' ' ')
  echo "$name|$form|$count|$first"
}
echo "# NUM HEAD $(git -C "$NUM" rev-parse HEAD)"
# Identities, files and hash domains
for n in RP-PREPARED-COMBINATION-DUAL-v1 retained_precision_prepared_combination_v1 \
         retained_precision_combination_formation_v1 receipt_bindings; do check bare "$n"; done
# Wire values (quoted JSON strings) new in B2
for n in mechanics_combination pre_source_refusal operand_validation combined_preparation no_selected_operand \
         retained_selected retained_unavailable base_withheld no_retained_mechanics combination_unresolved \
         operand_source_unavailable operand_preparation_failure combination_operand result_state_subtraction \
         range_envelope mechanics; do check quoted "\"$n\""; done
# Wire members (quoted keys) new in B2
for n in operand_preparations requested_operands representative_source_ref combination_index combination_id \
         call_ref result_ids disposition expression operand_preparation_ref operand_index selected_run imports \
         requested_by purpose minuend_id subtrahend_id operand_ids terms factor ledger_sha256; do check quoted "\"$n\""; done
# SCHEMA $def names
for n in Combination CombinationAttempt CombinationExpression CombinationGroup CombinationReason CombinationSource \
         MechanicsCombinationCall OperandPreparation; do check bare "$n"; done
# Internal Rust names proposed for lanes P and A (not wire values)
for n in CombinationCustody CaseEquivalents CombinationTerms RangeOperands CombinationIds COMBINATION_SELECTED_MESSAGE \
         COMBINATION_UNAVAILABLE_MESSAGE COMBINATION_DEFINITION_ID COMBINATION_DEFINITION_HASH \
         RETAINED_PRECISION_COMBINATION; do check bare "$n"; done
