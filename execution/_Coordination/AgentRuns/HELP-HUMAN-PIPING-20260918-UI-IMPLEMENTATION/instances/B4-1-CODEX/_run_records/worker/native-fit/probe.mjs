import { createRequire } from 'node:module';
import { spawn } from 'node:child_process';
import { writeFile, mkdir, open } from 'node:fs/promises';
import path from 'node:path';
const desktop=process.cwd(); const out=process.argv[2]; await mkdir(out,{recursive:true});
const require=createRequire(path.join(desktop,'package.json')); const { chromium }=require('playwright');
const executable='/Users/ryan/Library/Caches/ms-playwright/chromium-1223/chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing';
const log=await open(path.join(out,'vite.txt'),'w');
const server=spawn('npm',['run','dev','--','--port','5174'],{cwd:desktop,detached:true,stdio:['ignore',log.fd,log.fd]}); let browser;
try {
 for(let i=0;i<100;i++){try{if((await fetch('http://127.0.0.1:5174')).ok)break;}catch{} await new Promise(r=>setTimeout(r,100));}
 browser=await chromium.launch({executablePath:executable,headless:true}); const context=await browser.newContext({viewport:{width:1280,height:800}}); await context.tracing.start({screenshots:true,snapshots:true,sources:true}); const page=await context.newPage();
 await page.goto('http://127.0.0.1:5174'); await page.getByTestId('workspace-toolbar').waitFor();
 await page.getByTestId('view-switch-both').click();
 if(await page.getByTestId('toggle-inspector').getAttribute('aria-expanded')!=='true') await page.getByTestId('toggle-inspector').click();
 await page.getByTestId('layout-mode-grid').click(); const grid=page.getByRole('grid',{name:'Node coordinates'});
 if(process.argv[3]) await page.addStyleTag({content:process.argv[3]});
 const measurements=async()=>page.evaluate(()=>{
   const grid=document.querySelector('.engineering-table [role=grid]'); let e=grid; const ancestors=[];
   while(e){const r=e.getBoundingClientRect(),s=getComputedStyle(e); ancestors.push({tag:e.tagName,cls:e.className,role:e.getAttribute('role'),clientWidth:e.clientWidth,scrollWidth:e.scrollWidth,scrollLeft:e.scrollLeft,rect:{x:r.x,y:r.y,width:r.width,height:r.height},display:s.display,minWidth:s.minWidth,width:s.width,overflowX:s.overflowX,columns:s.gridTemplateColumns});e=e.parentElement;}
   const named={}; for(const [name,selector] of Object.entries({header:'.engineering-table-header',rows:'[data-testid=engineering-table-rows]',row:'.engineering-table [role=rowgroup] [role=row]',filter:'[data-testid=model-tree-filter-input]',tabs:'.entity-grid-tabs',footer:'.engineering-table-footer',z:'[data-testid="table-cell-node:N-100-z"]'})){const n=document.querySelector(selector),r=n.getBoundingClientRect();named[name]={clientWidth:n.clientWidth,scrollWidth:n.scrollWidth,scrollLeft:n.scrollLeft,x:r.x,y:r.y,width:r.width,height:r.height};}
   return {ancestors,named};
 });
 const before=await measurements(); await page.screenshot({path:path.join(out,'before.png')});
 await grid.hover({position:{x:Math.min(200,(await grid.boundingBox()).width-10),y:100}}); await page.mouse.wheel(700,0); await page.waitForTimeout(300);
 const after=await measurements(); await page.screenshot({path:path.join(out,'after-wheel.png')});
 await writeFile(path.join(out,'measurements.json'),JSON.stringify({temporaryStyles:process.argv[3]??null,executable,version:browser.version(),viewport:page.viewportSize(),before,after},null,2));
 await context.tracing.stop({path:path.join(out,'trace.zip')}); console.log(JSON.stringify({gridBefore:before.ancestors[0],gridAfter:after.ancestors[0],zAfter:after.named.z,filterBefore:before.named.filter,filterAfter:after.named.filter},null,2));
}finally{if(browser)await browser.close();try{process.kill(-server.pid,'SIGTERM');}catch{}await log.close();}
