import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { ControlledExportLink, routeBindingForTestId } from "./ControlledExportLink";

import { isNativeResultSaveRuntime, saveNativeResultJson } from "../result-export/nativeResultSave";
vi.mock("../result-export/nativeResultSave", () => ({ isNativeResultSaveRuntime: vi.fn(() => false), saveNativeResultJson: vi.fn() }));
afterEach(() => { vi.mocked(isNativeResultSaveRuntime).mockReturnValue(false); vi.mocked(saveNativeResultJson).mockReset(); });

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


describe("native canonical save control", () => {
 const name="openpipestress-preview-results-run-350.json";
 const payload={private_payload_included:false,protected_content_included:false,project_name:"Invented local project",opaque_leaf:"unknown retained locally"};
 const href=`data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload,null,4)+"\n")}`;
 const receipt={outcome:"saved" as const,file_name:name,byte_count:href.length,replaced_existing:false as const,durability:"not_guaranteed" as const,path_containment:"best_effort_non_adversarial" as const};
 it("native has no href/download/anchor handlers and synchronous busy blocks duplicate activation",async()=>{
  vi.mocked(isNativeResultSaveRuntime).mockReturnValue(true);
  let finish!:(value:typeof receipt)=>void;vi.mocked(saveNativeResultJson).mockImplementation(()=>new Promise(resolve=>{finish=resolve;}));
  const anchorClick=vi.fn(),anchorKey=vi.fn(),token={};
  render(<ControlledExportLink data-testid="result-export-link" href={href} download={name} nativeCurrentBinding={token} onClick={anchorClick} onKeyDown={anchorKey}>Save native</ControlledExportLink>);
  const button=screen.getByRole("button",{name:"Save native"});expect(button).toBeDisabled();expect(button).not.toHaveAttribute("href");expect(button).not.toHaveAttribute("download");expect(button).toHaveAttribute("type","button");
  fireEvent.click(button);expect(saveNativeResultJson).not.toHaveBeenCalled();fireEvent.click(screen.getByTestId("result-export-link-local-private-intent"));expect(button).toBeEnabled();
  act(()=>{fireEvent.click(button);fireEvent.click(button);fireEvent.keyDown(button,{key:"Enter"});});
  expect(saveNativeResultJson).toHaveBeenCalledTimes(1);expect(anchorClick).not.toHaveBeenCalled();expect(anchorKey).not.toHaveBeenCalled();expect(button).toBeDisabled();expect(screen.getByRole("status")).toHaveTextContent("Saving");
  expect(saveNativeResultJson).toHaveBeenCalledWith({href,file_name:name,screening:{route_id:"DOTH-JSON-001",export_context:"local_private",explicit_local_private_intent:true,blocked:false,materialization_withheld:false,lossless_required:true,exact_payload_match:true,blocking_count:0},local_first:{route_id:"DOTH-JSON-001",export_context:"local_private",storage_context:"local_private",action:"include_metadata_only",reason_code:"PRIVATE_LOCAL_METADATA_ALLOWED",blocked:false,metadata_only:true,explicit_local_private_intent:true}});
  expect(screen.getByTestId("result-export-link-redaction-findings")).toHaveTextContent("WARNING");
  await act(async()=>{finish(receipt);});expect(screen.getByRole("status")).toHaveTextContent(`Saved ${name}`);expect(button).toBeEnabled();
 });
 it("rejected IPC is an error with conservative partial status and never an HTML fallback",async()=>{
  vi.mocked(isNativeResultSaveRuntime).mockReturnValue(true);vi.mocked(saveNativeResultJson).mockRejectedValue({code:"WRITE_FAILED",stage:"write",message:"failure",partial_file_name:name,cleanup:"retained"});
  render(<ControlledExportLink data-testid="result-export-link" href={href} download={name} nativeCurrentBinding={{}}>Save native</ControlledExportLink>);fireEvent.click(screen.getByTestId("result-export-link-local-private-intent"));fireEvent.click(screen.getByRole("button",{name:"Save native"}));await waitFor(()=>expect(screen.getByRole("status")).toHaveTextContent("Save failed (write); cleanup=retained"));expect(screen.getByRole("status")).toHaveTextContent(name);expect(screen.queryByRole("link")).toBeNull();expect(screen.getByRole("button")).not.toHaveAttribute("href");
 });
 it("href/name/binding/intent changes invalidate old completion without releasing in-flight admission",async()=>{
  vi.mocked(isNativeResultSaveRuntime).mockReturnValue(true);
  for(const change of ["href","name","binding","intent"]){
   let finish!:(value:typeof receipt)=>void;vi.mocked(saveNativeResultJson).mockImplementation(()=>new Promise(resolve=>{finish=resolve;}));const token={};
   const view=render(<ControlledExportLink data-testid="result-export-link" href={href} download={name} nativeCurrentBinding={token}>Save</ControlledExportLink>);fireEvent.click(screen.getByTestId("result-export-link-local-private-intent"));fireEvent.click(screen.getByRole("button",{name:"Save"}));
   if(change==="intent")fireEvent.click(screen.getByTestId("result-export-link-local-private-intent"));else view.rerender(<ControlledExportLink data-testid="result-export-link" href={change==="href"?href+"%20":href} download={change==="name"?"openpipestress-preview-results-run-500.json":name} nativeCurrentBinding={change==="binding"?{}:token}>Save</ControlledExportLink>);
   expect(screen.getByRole("button")).toBeDisabled();const calls=vi.mocked(saveNativeResultJson).mock.calls.length;fireEvent.click(screen.getByRole("button"));expect(saveNativeResultJson).toHaveBeenCalledTimes(calls);
   await act(async()=>{finish(receipt);});expect(screen.getByRole("status")).not.toHaveTextContent("Saved");view.unmount();
  }
 });
 it("unmount suppresses completion and missing Current binding cannot invoke",async()=>{
  vi.mocked(isNativeResultSaveRuntime).mockReturnValue(true);let finish!:(value:typeof receipt)=>void;vi.mocked(saveNativeResultJson).mockImplementation(()=>new Promise(resolve=>{finish=resolve;}));
  const view=render(<ControlledExportLink data-testid="result-export-link" href={href} download={name} nativeCurrentBinding={{}}>Save</ControlledExportLink>);fireEvent.click(screen.getByTestId("result-export-link-local-private-intent"));fireEvent.click(screen.getByRole("button"));view.unmount();await act(async()=>{finish(receipt);});expect(screen.queryByRole("status")).toBeNull();
  render(<ControlledExportLink data-testid="result-export-link" href={href} download={name}>Save</ControlledExportLink>);fireEvent.click(screen.getByTestId("result-export-link-local-private-intent"));expect(screen.getByRole("button")).toBeDisabled();
 });
 it("blocked/protected/rejected/quarantined/secret and intent-stripped native materialization never invoke",()=>{
  vi.mocked(isNativeResultSaveRuntime).mockReturnValue(true);
  for(const facts of [{privacy_classification:"protected_code_data",redistribution_status:"protected_suspected",review_status:"accepted"},{privacy_classification:"public_metadata",redistribution_status:"public_permissive",review_status:"rejected"},{privacy_classification:"public_metadata",redistribution_status:"public_permissive",review_status:"quarantined"},{privacy_classification:"secret_like_data",redistribution_status:"private_only",review_status:"accepted",secret_material_present:true},{local_private_intent:true}]){
   const data={field_id:"blocked",field_class:"test",...facts,value:"INVENTED_SENSITIVE"};const view=render(<ControlledExportLink data-testid="result-export-link" href={`data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(data))}`} download={name} nativeCurrentBinding={{}}>Save</ControlledExportLink>);fireEvent.click(screen.getByTestId("result-export-link-local-private-intent"));expect(screen.getByRole("button")).toBeDisabled();fireEvent.click(screen.getByRole("button"));expect(saveNativeResultJson).not.toHaveBeenCalled();expect(screen.queryByRole("link")).toBeNull();view.unmount();
  }
 });
});
