import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { ControlledExportLink, routeBindingForTestId } from "./ControlledExportLink";

describe("ControlledExportLink", () => {
  it("uses fixed route contexts", () => {
    expect(routeBindingForTestId("report-export-link")).toMatchObject({
      routeId: "DREP-JSON-002",
      context: "public_report"
    });
    expect(routeBindingForTestId("caepipe-external-export-link")).toMatchObject({
      routeId: "DOTH-CAEPIPE-LOCAL-006",
      context: "local_private"
    });
    expect(routeBindingForTestId("native-package-link")).toEqual({
      routeId: "DOTH-HANDOFF-002",
      context: "downstream_tool"
    });
  });

  it("ignores source intent and requires wrapper-owned intent for known private values", () => {
    const payload = {
      private_payload_included: false,
      protected_content_included: false,
      project_name: "Invented local project",
      local_private_intent: true,
      export_policy: { explicit_local_private_intent: true }
    };
    render(
      <ControlledExportLink
        data-testid="secret-private-library-export-link"
        download="private.json"
        href={`data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload))}`}
      >
        Private JSON
      </ControlledExportLink>
    );
    expect(screen.getByTestId("secret-private-library-export-link").getAttribute("href")).toBeNull();
    expect(screen.getByTestId("secret-private-library-export-link").closest("[data-route-id]")).toHaveAttribute(
      "data-local-first-reason",
      "LOCAL_PRIVATE_INTENT_REQUIRED"
    );
    fireEvent.click(screen.getByTestId("secret-private-library-export-link-local-private-intent"));
    expect(screen.getByTestId("secret-private-library-export-link").getAttribute("href")).toContain("data:application/json");
    expect(screen.getByTestId("secret-private-library-export-link").closest("[data-route-id]")).toHaveAttribute(
      "data-local-first-reason",
      "PRIVATE_LOCAL_METADATA_ALLOWED"
    );
  });

  it("binds the native package to downstream-tool control without local-private fallback", () => {
    render(
      <ControlledExportLink
        data-testid="native-package-link"
        download="native-package.json"
        href={`data:application/json;charset=utf-8,${encodeURIComponent(
          JSON.stringify({
            private_payload_included: false,
            protected_content_included: false,
            opaque_leaf: "Invented value without leaf metadata"
          })
        )}`}
      >
        Native package
      </ControlledExportLink>
    );

    expect(screen.getByTestId("native-package-link").closest("[data-route-id]")).toHaveAttribute(
      "data-route-id",
      "DOTH-HANDOFF-002"
    );
    expect(screen.getByTestId("native-package-link").closest("[data-route-id]")).toHaveAttribute(
      "data-local-first-reason",
      "SAFE_PUBLIC_METADATA"
    );
    expect(screen.queryByTestId("native-package-link-local-private-intent")).not.toBeInTheDocument();
    expect(screen.getByTestId("native-package-link-redaction-summary")).toHaveTextContent("blocked=false");
  });

  it("requires the Parser CSV link's own intent and exposes no side effect before it", () => {
    const csv = "section,stable_id,load_case,value\nELEMENT_FORCES,E-1,LC-1,12.5\n";
    render(
      <>
        <ControlledExportLink
          data-testid="caepipe-external-export-link"
          download="harness.json"
          href={`data:application/json;charset=utf-8,${encodeURIComponent(
            JSON.stringify({ project_name: "Invented private project" })
          )}`}
        >
          Harness JSON
        </ControlledExportLink>
        <ControlledExportLink
          data-testid="caepipe-external-csv-link"
          download="parser.csv"
          href={`data:text/csv;charset=utf-8,${encodeURIComponent(csv)}`}
        >
          Parser CSV
        </ControlledExportLink>
      </>
    );

    expect(screen.getByTestId("caepipe-external-csv-link")).toHaveAttribute("aria-disabled", "true");
    expect(screen.getByTestId("caepipe-external-csv-link")).not.toHaveAttribute("href");
    fireEvent.click(screen.getByTestId("caepipe-external-export-link-local-private-intent"));
    expect(screen.getByTestId("caepipe-external-export-link")).toHaveAttribute("href");
    expect(screen.getByTestId("caepipe-external-csv-link")).not.toHaveAttribute("href");
    fireEvent.click(screen.getByTestId("caepipe-external-csv-link"));
    expect(screen.getByTestId("caepipe-external-csv-link")).not.toHaveAttribute("href");

    fireEvent.click(screen.getByTestId("caepipe-external-csv-link-local-private-intent"));
    const csvHref = screen.getByRole("link", { name: "Parser CSV" }).getAttribute("href") ?? "";
    expect(decodeURIComponent(csvHref.split(",", 2)[1])).toBe(csv);
    expect(screen.getByTestId("caepipe-external-csv-link-redaction-decisions")).toHaveTextContent(
      "classification=private_project_data"
    );
    expect(screen.getByTestId("caepipe-external-csv-link-redaction-findings")).toHaveTextContent(
      "severity=WARNING"
    );
  });

  it("renders sanitized per-item evidence before allowed and blocked exposure controls", () => {
    const rawPrivate = "INVENTED_PRIVATE_VALUE_MUST_NOT_RENDER_IN_EVIDENCE";
    const { rerender } = render(
      <ControlledExportLink
        data-testid="native-package-link"
        download="native.json"
        href={`data:application/json;charset=utf-8,${encodeURIComponent(
          JSON.stringify({ opaque_leaf: "Invented opaque value" })
        )}`}
      >
        Native package
      </ControlledExportLink>
    );
    const allowedDecisions = screen.getByLabelText("native-package-link redaction decisions");
    const allowedFindings = screen.getByLabelText("native-package-link redaction findings");
    const allowedLink = screen.getByRole("link", { name: "Native package" });
    expect(allowedDecisions).toHaveTextContent("path=");
    expect(allowedDecisions).toHaveTextContent("classification=unknown");
    expect(allowedDecisions).toHaveTextContent("action=redact_value");
    expect(allowedFindings).toHaveTextContent("severity=WARNING");
    expect(allowedDecisions.compareDocumentPosition(allowedLink) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(allowedFindings.compareDocumentPosition(allowedLink) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();

    rerender(
      <ControlledExportLink
        data-testid="secret-private-library-export-link"
        download="private.json"
        href={`data:application/json;charset=utf-8,${encodeURIComponent(
          JSON.stringify({ project_name: rawPrivate })
        )}`}
      >
        Private JSON
      </ControlledExportLink>
    );
    const blockedControl = screen.getByTestId("secret-private-library-export-link");
    const blockedDecisions = screen.getByLabelText("secret-private-library-export-link redaction decisions");
    const blockedFindings = screen.getByLabelText("secret-private-library-export-link redaction findings");
    expect(blockedControl).toHaveAttribute("aria-disabled", "true");
    expect(blockedControl).not.toHaveAttribute("href");
    expect(blockedDecisions).toHaveTextContent("action=block_export");
    expect(blockedFindings).toHaveTextContent("severity=BLOCKING");
    expect(blockedDecisions).not.toHaveTextContent(rawPrivate);
    expect(blockedFindings).not.toHaveTextContent(rawPrivate);
    expect(blockedDecisions.compareDocumentPosition(blockedControl) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(blockedFindings.compareDocumentPosition(blockedControl) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
  });
});

describe('canonical local result link',()=>{
 it('requires its own intent then exposes original serialized bytes with local unknown/private warnings',()=>{
  const payload={private_payload_included:false,protected_content_included:false,project_name:'Invented private project',opaque_leaf:'unknown retained locally'};
  const href=`data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload,null,4)+'\n')}`;
  render(<ControlledExportLink data-testid="result-export-link" href={href}>Local canonical JSON</ControlledExportLink>);
  expect(routeBindingForTestId('result-export-link')).toMatchObject({routeId:'DOTH-JSON-001',context:'local_private',lossless:true});
  expect(screen.getByTestId('result-export-link')).not.toHaveAttribute('href');fireEvent.click(screen.getByTestId('result-export-link-local-private-intent'));
  expect(screen.getByTestId('result-export-link')).toHaveAttribute('href',href);expect(screen.getByTestId('result-export-link-redaction-findings')).toHaveTextContent('WARNING');
 });
 it('blocks route-intent stripping from unknown raw annotations despite lossless policy materialization',()=>{
  const payload={private_payload_included:false,protected_content_included:false,result_envelope:{source_annotations:[{metadata:{component:'axial_force',local_private_intent:true,unknown:'retained'}}]}};
  render(<ControlledExportLink data-testid="result-export-link" href={`data:application/json,${encodeURIComponent(JSON.stringify(payload))}`}>Canonical JSON</ControlledExportLink>);
  fireEvent.click(screen.getByTestId('result-export-link-local-private-intent'));expect(screen.getByTestId('result-export-link')).not.toHaveAttribute('href');expect(screen.getByTestId('result-export-link-canonical-block-reason')).toHaveTextContent('CANONICAL_PAYLOAD_MATERIALIZATION_CHANGED');
 });
 it('source intent cannot grant access and policy-blocked protected/rejected/quarantined/secret facts remain blocked',()=>{
  const {rerender}=render(<ControlledExportLink data-testid="result-export-link" href={`data:application/json,${encodeURIComponent(JSON.stringify({local_private_intent:true,export_policy:{explicit_local_private_intent:true},opaque:'source cannot grant intent'}))}`}>Canonical</ControlledExportLink>);
  expect(screen.getByTestId('result-export-link')).not.toHaveAttribute('href');
  for(const facts of [{privacy_classification:'protected_code_data',redistribution_status:'protected_suspected',review_status:'accepted'},{privacy_classification:'public_metadata',redistribution_status:'public_permissive',review_status:'rejected'},{privacy_classification:'public_metadata',redistribution_status:'public_permissive',review_status:'quarantined'},{privacy_classification:'secret_like_data',redistribution_status:'private_only',review_status:'accepted',secret_material_present:true}]){
   const payload={field_id:'policy-blocked',field_class:'test',...facts,value:'INVENTED_SENSITIVE_SCALAR'};rerender(<ControlledExportLink data-testid="result-export-link" href={`data:application/json,${encodeURIComponent(JSON.stringify(payload))}`}>Canonical</ControlledExportLink>);const intent=screen.getByTestId('result-export-link-local-private-intent') as HTMLInputElement;if(!intent.checked)fireEvent.click(intent);expect(screen.getByTestId('result-export-link')).not.toHaveAttribute('href');
  }
 });
});
