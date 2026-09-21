import { createRequire } from 'node:module';
import { spawn, execFileSync } from 'node:child_process';
import { writeFile, mkdir, open, readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import path from 'node:path';
const out=path.resolve(process.argv[2]), desktop=process.cwd(); await mkdir(out,{recursive:true});
const require=createRequire(path.join(desktop,'package.json')); const {chromium}=require('playwright');
const executable='/Users/ryan/Library/Caches/ms-playwright/chromium-1223/chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing';
const log=await open(path.join(out,'vite.txt'),'w'); const server=spawn('npm',['run','dev','--','--port','5174'],{cwd:desktop,detached:true,stdio:['ignore',log.fd,log.fd]}); let browser;
try {
 for(let i=0;i<100;i++){try{if((await fetch('http://127.0.0.1:5174')).ok)break;}catch{}await new Promise(r=>setTimeout(r,100));}
 browser=await chromium.launch({executablePath:executable,headless:true});
 const run={source:execFileSync('git',['rev-parse','HEAD'],{encoding:'utf8'}).trim(),cwd:desktop,argv:process.argv,executable,version:browser.version(),executableSha256:createHash('sha256').update(await readFile(executable)).digest('hex'),scriptSha256:createHash('sha256').update(await readFile(process.argv[1])).digest('hex'),scenarios:[]};
 for(const [profile,viewport] of [['desktop',{width:1440,height:920}],['compact',{width:1280,height:800}]]){
  for(const target of ['grid-body','family-controls']){
   const name=`${profile}-${target}`,context=await browser.newContext({viewport}); await context.tracing.start({screenshots:true,snapshots:true,sources:true}); const page=await context.newPage();
   await page.goto('http://127.0.0.1:5174');await page.getByTestId('workspace-toolbar').waitFor();await page.getByTestId('view-switch-both').click();
   if(await page.getByTestId('toggle-inspector').getAttribute('aria-expanded')!=='true')await page.getByTestId('toggle-inspector').click();
   await page.getByTestId('layout-mode-grid').click();
   const measure=()=>page.evaluate(()=>{
    const info=e=>{const r=e.getBoundingClientRect(),s=getComputedStyle(e);return{tag:e.tagName,cls:e.className,role:e.getAttribute('role'),clientWidth:e.clientWidth,clientHeight:e.clientHeight,scrollWidth:e.scrollWidth,scrollHeight:e.scrollHeight,scrollTop:e.scrollTop,scrollLeft:e.scrollLeft,bounds:{x:r.x,y:r.y,width:r.width,height:r.height,right:r.right,bottom:r.bottom},overflowX:s.overflowX,overflowY:s.overflowY,height:s.height,minHeight:s.minHeight,display:s.display,flex:s.flex};};
    const named={};for(const [name,selector] of Object.entries({modelPane:'.shell-table-pane',treeHost:'.shell-tree-host',modelTree:'.model-tree',table:'.engineering-table',grid:'.engineering-table [role=grid]',rowgroup:'[data-testid=engineering-table-rows]',rowContent:'[data-testid=engineering-table-rows] > div',filter:'[data-testid=model-tree-filter-input]',families:'.entity-grid-tabs',header:'.engineering-table-header',footer:'.engineering-table-footer',review:'.entity-grid-review',viewportHost:'[data-testid=viewport-canvas]',drawnCanvas:'[data-testid=viewport-canvas] canvas'})){const e=document.querySelector(selector);named[name]=e?info(e):null;}
    const ancestry=[];let e=document.querySelector('[data-testid=engineering-table-rows]');while(e){ancestry.push(info(e));e=e.parentElement;}
    return{named,ancestry,rowCount:document.querySelectorAll('[data-testid=engineering-table-rows] [role=row]').length,activeElement:document.activeElement?.outerHTML.slice(0,400)};
   });
   const before=await measure();await page.screenshot({path:path.join(out,`${name}-before.png`)});
   const locator=target==='grid-body'?page.getByTestId('engineering-table-rows'):page.locator('.entity-grid-tabs');const b=await locator.boundingBox();const pointer={x:b.x+Math.min(100,b.width/2),y:b.y+Math.min(target==='grid-body'?100:12,b.height/2)};
   await page.mouse.move(pointer.x,pointer.y);const hit=await page.evaluate(({x,y})=>document.elementFromPoint(x,y)?.outerHTML.slice(0,600),pointer);await page.mouse.wheel(0,600);await page.waitForTimeout(350);
   const after=await measure();await page.screenshot({path:path.join(out,`${name}-after.png`)});await context.tracing.stop({path:path.join(out,`${name}-trace.zip`)});
   const evidence={profile,viewport,target,input:{pointer,hit,deltaX:0,deltaY:600,method:'Playwright page.mouse.wheel; no scroll setter/style injection'},before,after};await writeFile(path.join(out,`${name}.json`),JSON.stringify(evidence,null,2));run.scenarios.push({name,input:evidence.input,changed:after.ancestry.filter((n,i)=>n.scrollTop!==before.ancestry[i].scrollTop||n.scrollLeft!==before.ancestry[i].scrollLeft).map((n,i)=>({tag:n.tag,cls:n.cls,scrollTop:n.scrollTop,scrollLeft:n.scrollLeft})),filterY:[before.named.filter.bounds.y,after.named.filter.bounds.y],familiesY:[before.named.families.bounds.y,after.named.families.bounds.y],rowgroup:{clientHeight:before.named.rowgroup.clientHeight,scrollHeight:before.named.rowgroup.scrollHeight,contentHeight:before.named.rowContent.bounds.height,rowCount:before.rowCount}});await context.close();
  }
 }
 await writeFile(path.join(out,'RUN.json'),JSON.stringify(run,null,2));console.log(JSON.stringify(run.scenarios,null,2));
}finally{if(browser)await browser.close();try{process.kill(-server.pid,'SIGTERM');}catch{}await log.close();}
