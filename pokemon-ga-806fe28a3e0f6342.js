let U=0,P=null,R=`undefined`,Y=`boolean`,N=128,Z=`string`,Q=1,_=`Object`,S=`utf-8`,X=`number`,a0=4,W=`function`,M=Array,T=Error,$=JSON.stringify,a2=Object,a1=Reflect,V=Uint8Array,O=undefined;var G=(async(a,b)=>{if(typeof Response===W&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===W){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var u=(a=>{const b=typeof a;if(b==X||b==Y||a==P){return `${a}`};if(b==Z){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==P){return `Symbol`}else{return `Symbol(${b})`}};if(b==W){const b=a.name;if(typeof b==Z&&b.length>U){return `Function(${b})`}else{return `Function`}};if(M.isArray(a)){const b=a.length;let c=`[`;if(b>U){c+=u(a[U])};for(let d=Q;d<b;d++){c+=`, `+ u(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>Q){d=c[Q]}else{return toString.call(a)};if(d==_){try{return `Object(`+ $(a)+ `)`}catch(a){return _}};if(a instanceof T){return `${a.name}: ${a.message}\n${a.stack}`};return d});var I=((a,b)=>{});var y=((c,d,e)=>{try{a.wasm_bindgen__convert__closures__invoke1_ref__h25c3b24a1ee67c32(c,d,x(e))}finally{b[w++]=O}});var F=((a,b)=>{a=a>>>U;const c=E();const d=c.subarray(a/a0,a/a0+ b);const e=[];for(let a=U;a<d.length;a++){e.push(f(d[a]))};return e});var g=(a=>{if(d===b.length)b.push(b.length+ Q);const c=d;d=b[c];b[c]=a;return c});var f=(a=>{const b=c(a);e(a);return b});var B=((c,d,e)=>{try{a.wasm_bindgen__convert__closures__invoke1_mut_ref__h56f0bd9a1f4059b1(c,d,x(e))}finally{b[w++]=O}});var L=(async(b)=>{if(a!==O)return a;if(typeof b===R){b=new URL(`pokemon-ga-806fe28a3e0f6342_bg.wasm`,import.meta.url)};const c=H();if(typeof b===Z||typeof Request===W&&b instanceof Request||typeof URL===W&&b instanceof URL){b=fetch(b)};I(c);const {instance:d,module:e}=await G(await b,c);return J(d,e)});function C(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(g(b))}}var K=(b=>{if(a!==O)return a;const c=H();I(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return J(d,b)});var A=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he5a13ab9feccda9d(b,c,g(d))});var p=(()=>{if(o===P||o.byteLength===U){o=new Int32Array(a.memory.buffer)};return o});var l=(a=>a===O||a===P);var c=(a=>b[a]);var J=((b,c)=>{a=b.exports;L.__wbindgen_wasm_module=c;m=P;o=P;D=P;i=P;a.__wbindgen_start();return a});var H=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==Q){b.a=U;return !0};const c=!1;return c});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return g(b)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=k(a,b);return g(c)});b.wbg.__wbg_fetch_6a2624d7f767e331=(a=>{const b=fetch(c(a));return g(b)});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===X?d:O;n()[a/8+ Q]=l(e)?U:e;p()[a/a0+ U]=!l(e)});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===Z;return b});b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===Z?e:O;var g=l(f)?U:t(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=q;p()[b/a0+ Q]=h;p()[b/a0+ U]=g});b.wbg.__wbg_cachekey_b81c1aacc6a0645c=((a,b)=>{const d=c(b).__yew_subtree_cache_key;p()[a/a0+ Q]=l(d)?U:d;p()[a/a0+ U]=!l(d)});b.wbg.__wbg_subtreeid_e80a1798fee782f9=((a,b)=>{const d=c(b).__yew_subtree_id;p()[a/a0+ Q]=l(d)?U:d;p()[a/a0+ U]=!l(d)});b.wbg.__wbg_setsubtreeid_e1fab6b578c800cf=((a,b)=>{c(a).__yew_subtree_id=b>>>U});b.wbg.__wbg_setcachekey_75bcd45312087529=((a,b)=>{c(a).__yew_subtree_cache_key=b>>>U});b.wbg.__wbg_listenerid_6dcf1c62b7b7de58=((a,b)=>{const d=c(b).__yew_listener_id;p()[a/a0+ Q]=l(d)?U:d;p()[a/a0+ U]=!l(d)});b.wbg.__wbg_setlistenerid_f2e783343fa0cec1=((a,b)=>{c(a).__yew_listener_id=b>>>U});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new T();return g(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{let d;let e;try{d=b;e=c;console.error(k(b,c))}finally{a.__wbindgen_free(d,e,Q)}});b.wbg.__wbg_queueMicrotask_118eeb525d584d9a=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_26a89c14c53809c0=(a=>{const b=c(a).queueMicrotask;return g(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===W;return b});b.wbg.__wbindgen_error_new=((a,b)=>{const c=new T(k(a,b));return g(c)});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==P;return d});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===O;return b});b.wbg.__wbindgen_in=((a,b)=>{const d=c(a) in c(b);return d});b.wbg.__wbg_crypto_d05b68a3572bb8ca=(a=>{const b=c(a).crypto;return g(b)});b.wbg.__wbg_process_b02b3570280d0366=(a=>{const b=c(a).process;return g(b)});b.wbg.__wbg_versions_c1cb42213cedf0f5=(a=>{const b=c(a).versions;return g(b)});b.wbg.__wbg_node_43b1089f407e4ec2=(a=>{const b=c(a).node;return g(b)});b.wbg.__wbg_msCrypto_10fc94afee92bd76=(a=>{const b=c(a).msCrypto;return g(b)});b.wbg.__wbg_require_9a7e0f667ead4995=function(){return C((()=>{const a=module.require;return g(a)}),arguments)};b.wbg.__wbg_randomFillSync_b70ccbdf4926a99d=function(){return C(((a,b)=>{c(a).randomFillSync(f(b))}),arguments)};b.wbg.__wbg_getRandomValues_7e42b4fb8779dc6d=function(){return C(((a,b)=>{c(a).getRandomValues(c(b))}),arguments)};b.wbg.__wbindgen_jsval_loose_eq=((a,b)=>{const d=c(a)==c(b);return d});b.wbg.__wbindgen_boolean_get=(a=>{const b=c(a);const d=typeof b===Y?(b?Q:U):2;return d});b.wbg.__wbindgen_as_number=(a=>{const b=+c(a);return b});b.wbg.__wbindgen_number_new=(a=>{const b=a;return g(b)});b.wbg.__wbg_getwithrefkey_4a92a5eca60879b9=((a,b)=>{const d=c(a)[c(b)];return g(d)});b.wbg.__wbg_set_9182712abebf82ef=((a,b,d)=>{c(a)[f(b)]=f(d)});b.wbg.__wbg_error_a526fb08a0205972=((b,c)=>{var d=F(b,c).slice();a.__wbindgen_free(b,c*a0,a0);console.error(...d)});b.wbg.__wbg_instanceof_Window_99dc9805eaa2614b=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_5257b70811e953c0=(a=>{const b=c(a).document;return l(b)?U:g(b)});b.wbg.__wbg_location_0f233324e8e8c699=(a=>{const b=c(a).location;return g(b)});b.wbg.__wbg_history_370f36be0803466b=function(){return C((a=>{const b=c(a).history;return g(b)}),arguments)};b.wbg.__wbg_body_3eb73da919b867a1=(a=>{const b=c(a).body;return l(b)?U:g(b)});b.wbg.__wbg_head_267359b89c3f0368=(a=>{const b=c(a).head;return l(b)?U:g(b)});b.wbg.__wbg_createElement_1a136faad4101f43=function(){return C(((a,b,d)=>{const e=c(a).createElement(k(b,d));return g(e)}),arguments)};b.wbg.__wbg_createElementNS_d47e0c50fa2904e0=function(){return C(((a,b,d,e,f)=>{const h=c(a).createElementNS(b===U?O:k(b,d),k(e,f));return g(h)}),arguments)};b.wbg.__wbg_createTextNode_dbdd908f92bae1b1=((a,b,d)=>{const e=c(a).createTextNode(k(b,d));return g(e)});b.wbg.__wbg_getElementsByTagName_27819c43f010d576=((a,b,d)=>{const e=c(a).getElementsByTagName(k(b,d));return g(e)});b.wbg.__wbg_querySelector_d86f889797c65e88=function(){return C(((a,b,d)=>{const e=c(a).querySelector(k(b,d));return l(e)?U:g(e)}),arguments)};b.wbg.__wbg_instanceof_Element_f614cf57d4316979=(a=>{let b;try{b=c(a) instanceof Element}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_namespaceURI_0819c2800784a176=((b,d)=>{const e=c(d).namespaceURI;var f=l(e)?U:t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f});b.wbg.__wbg_setinnerHTML_99deeacfff0ae4cc=((a,b,d)=>{c(a).innerHTML=k(b,d)});b.wbg.__wbg_outerHTML_69934f9195df65af=((b,d)=>{const e=c(d).outerHTML;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f});b.wbg.__wbg_removeAttribute_5c264e727b67dbdb=function(){return C(((a,b,d)=>{c(a).removeAttribute(k(b,d))}),arguments)};b.wbg.__wbg_setAttribute_0918ea45d5a1c663=function(){return C(((a,b,d,e,f)=>{c(a).setAttribute(k(b,d),k(e,f))}),arguments)};b.wbg.__wbg_fetch_06d656a1b748ac0d=((a,b)=>{const d=c(a).fetch(c(b));return g(d)});b.wbg.__wbg_new_a979e9eedc5e81a3=function(){return C((()=>{const a=new Headers();return g(a)}),arguments)};b.wbg.__wbg_append_047382169b61373d=function(){return C(((a,b,d,e,f)=>{c(a).append(k(b,d),k(e,f))}),arguments)};b.wbg.__wbg_value_ab23a75318ea828f=((b,d)=>{const e=c(d).value;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f});b.wbg.__wbg_setvalue_918a8ae77531a942=((a,b,d)=>{c(a).value=k(b,d)});b.wbg.__wbg_href_1ab7f03b8a745310=function(){return C(((b,d)=>{const e=c(d).href;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f}),arguments)};b.wbg.__wbg_pathname_2cd8b46817926b06=function(){return C(((b,d)=>{const e=c(d).pathname;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f}),arguments)};b.wbg.__wbg_search_eb68df82d26f8761=function(){return C(((b,d)=>{const e=c(d).search;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f}),arguments)};b.wbg.__wbg_hash_9bd16c0f666cdf27=function(){return C(((b,d)=>{const e=c(d).hash;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f}),arguments)};b.wbg.__wbg_href_aa2244ca34a67d87=((b,d)=>{const e=c(d).href;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f});b.wbg.__wbg_pathname_d0d5b2fd2c7d8243=((b,d)=>{const e=c(d).pathname;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f});b.wbg.__wbg_search_b5c7b044aaf64616=((b,d)=>{const e=c(d).search;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f});b.wbg.__wbg_hash_286eced2921b7b34=((b,d)=>{const e=c(d).hash;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f});b.wbg.__wbg_sethash_cc573c60c5aedda2=((a,b,d)=>{c(a).hash=k(b,d)});b.wbg.__wbg_new_7d30e9d9d2deaf9d=function(){return C(((a,b)=>{const c=new URL(k(a,b));return g(c)}),arguments)};b.wbg.__wbg_newwithbase_1151109a3f062f92=function(){return C(((a,b,c,d)=>{const e=new URL(k(a,b),k(c,d));return g(e)}),arguments)};b.wbg.__wbg_setchecked_3b12f3d602a63e47=((a,b)=>{c(a).checked=b!==U});b.wbg.__wbg_value_c93cb4b4d352228e=((b,d)=>{const e=c(d).value;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f});b.wbg.__wbg_setvalue_9bd3f93b3864ddbf=((a,b,d)=>{c(a).value=k(b,d)});b.wbg.__wbg_item_b23b1deb13df4a81=((a,b)=>{const d=c(a).item(b>>>U);return l(d)?U:g(d)});b.wbg.__wbg_instanceof_Node_21fd40436f5d5572=(a=>{let b;try{b=c(a) instanceof Node}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_parentNode_f3957fdd408a62f7=(a=>{const b=c(a).parentNode;return l(b)?U:g(b)});b.wbg.__wbg_parentElement_86a7612dde875ba9=(a=>{const b=c(a).parentElement;return l(b)?U:g(b)});b.wbg.__wbg_childNodes_75d3da5f3a7bb985=(a=>{const b=c(a).childNodes;return g(b)});b.wbg.__wbg_firstChild_c3dac19eb85cc526=(a=>{const b=c(a).firstChild;return l(b)?U:g(b)});b.wbg.__wbg_lastChild_8f7b6f3825115eff=(a=>{const b=c(a).lastChild;return l(b)?U:g(b)});b.wbg.__wbg_nextSibling_13e9454ef5323f1a=(a=>{const b=c(a).nextSibling;return l(b)?U:g(b)});b.wbg.__wbg_setnodeValue_8656e865e9b11bbb=((a,b,d)=>{c(a).nodeValue=b===U?O:k(b,d)});b.wbg.__wbg_textContent_efe8338af53ddf62=((b,d)=>{const e=c(d).textContent;var f=l(e)?U:t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f});b.wbg.__wbg_settextContent_1fec240f77aa3dc4=((a,b,d)=>{c(a).textContent=b===U?O:k(b,d)});b.wbg.__wbg_appendChild_bd383ec5356c0bdb=function(){return C(((a,b)=>{const d=c(a).appendChild(c(b));return g(d)}),arguments)};b.wbg.__wbg_cloneNode_80501c66ab115588=function(){return C((a=>{const b=c(a).cloneNode();return g(b)}),arguments)};b.wbg.__wbg_insertBefore_882082ef4c5d7766=function(){return C(((a,b,d)=>{const e=c(a).insertBefore(c(b),c(d));return g(e)}),arguments)};b.wbg.__wbg_removeChild_14b08321b677677a=function(){return C(((a,b)=>{const d=c(a).removeChild(c(b));return g(d)}),arguments)};b.wbg.__wbg_instanceof_ShadowRoot_cb6366cb0956ce29=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_99e27ed8897850f2=(a=>{const b=c(a).host;return g(b)});b.wbg.__wbg_state_cabf8868613a7bdb=function(){return C((a=>{const b=c(a).state;return g(b)}),arguments)};b.wbg.__wbg_pushState_90b0a1cf59505502=function(){return C(((a,b,d,e,f,g)=>{c(a).pushState(c(b),k(d,e),f===U?O:k(f,g))}),arguments)};b.wbg.__wbg_newwithstrandinit_9fd2fc855c6327eb=function(){return C(((a,b,d)=>{const e=new Request(k(a,b),c(d));return g(e)}),arguments)};b.wbg.__wbg_instanceof_Response_0d25bb8436a9cefe=(a=>{let b;try{b=c(a) instanceof Response}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_url_47f8307501523859=((b,d)=>{const e=c(d).url;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f});b.wbg.__wbg_status_351700a30c61ba61=(a=>{const b=c(a).status;return b});b.wbg.__wbg_headers_e38c00d713e8888c=(a=>{const b=c(a).headers;return g(b)});b.wbg.__wbg_arrayBuffer_ec4617b29bb0f61c=function(){return C((a=>{const b=c(a).arrayBuffer();return g(b)}),arguments)};b.wbg.__wbg_target_791826e938c3e308=(a=>{const b=c(a).target;return l(b)?U:g(b)});b.wbg.__wbg_bubbles_f0783dc095f8e220=(a=>{const b=c(a).bubbles;return b});b.wbg.__wbg_cancelBubble_191799b8e0ab3254=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_d94a39b8c8f6eed1=(a=>{const b=c(a).composedPath();return g(b)});b.wbg.__wbg_preventDefault_d2c7416966cb0632=(a=>{c(a).preventDefault()});b.wbg.__wbg_addEventListener_1b158e9e95e0ab00=function(){return C(((a,b,d,e,f)=>{c(a).addEventListener(k(b,d),c(e),c(f))}),arguments)};b.wbg.__wbg_removeEventListener_177ff96081e6f22d=function(){return C(((a,b,d,e,f)=>{c(a).removeEventListener(k(b,d),c(e),f!==U)}),arguments)};b.wbg.__wbg_instanceof_HtmlLinkElement_c0e48d63cc52abe7=(a=>{let b;try{b=c(a) instanceof HTMLLinkElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_sethref_d02ad6d2474da457=((a,b,d)=>{c(a).href=k(b,d)});b.wbg.__wbg_setrel_8f1b970f9b1fd979=((a,b,d)=>{c(a).rel=k(b,d)});b.wbg.__wbg_settype_ea8511d9719a455f=((a,b,d)=>{c(a).type=k(b,d)});b.wbg.__wbg_signal_7876560d9d0f914c=(a=>{const b=c(a).signal;return g(b)});b.wbg.__wbg_new_fa36281638875de8=function(){return C((()=>{const a=new AbortController();return g(a)}),arguments)};b.wbg.__wbg_abort_7792bf3f664d7bb3=(a=>{c(a).abort()});b.wbg.__wbg_href_dc647488029294b4=((b,d)=>{const e=c(d).href;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f});b.wbg.__wbg_get_c43534c00f382c8a=((a,b)=>{const d=c(a)[b>>>U];return g(d)});b.wbg.__wbg_length_d99b680fd68bf71b=(a=>{const b=c(a).length;return b});b.wbg.__wbg_newnoargs_5859b6d41c6fe9f7=((a,b)=>{const c=new Function(k(a,b));return g(c)});b.wbg.__wbg_next_1938cf110c9491d4=(a=>{const b=c(a).next;return g(b)});b.wbg.__wbg_next_267398d0e0761bf9=function(){return C((a=>{const b=c(a).next();return g(b)}),arguments)};b.wbg.__wbg_done_506b44765ba84b9c=(a=>{const b=c(a).done;return b});b.wbg.__wbg_value_31485d8770eb06ab=(a=>{const b=c(a).value;return g(b)});b.wbg.__wbg_iterator_364187e1ee96b750=(()=>{const a=Symbol.iterator;return g(a)});b.wbg.__wbg_get_5027b32da70f39b1=function(){return C(((a,b)=>{const d=a1.get(c(a),c(b));return g(d)}),arguments)};b.wbg.__wbg_call_a79f1973a4f07d5e=function(){return C(((a,b)=>{const d=c(a).call(c(b));return g(d)}),arguments)};b.wbg.__wbg_new_87d841e70661f6e9=(()=>{const a=new a2();return g(a)});b.wbg.__wbg_self_086b5302bcafb962=function(){return C((()=>{const a=self.self;return g(a)}),arguments)};b.wbg.__wbg_window_132fa5d7546f1de5=function(){return C((()=>{const a=window.window;return g(a)}),arguments)};b.wbg.__wbg_globalThis_e5f801a37ad7d07b=function(){return C((()=>{const a=globalThis.globalThis;return g(a)}),arguments)};b.wbg.__wbg_global_f9a61fce4af6b7c1=function(){return C((()=>{const a=global.global;return g(a)}),arguments)};b.wbg.__wbg_from_a663e01d8dab8e44=(a=>{const b=M.from(c(a));return g(b)});b.wbg.__wbg_instanceof_ArrayBuffer_f4521cec1b99ee35=(a=>{let b;try{b=c(a) instanceof ArrayBuffer}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_call_f6a2bc58c19c53c6=function(){return C(((a,b,d)=>{const e=c(a).call(c(b),c(d));return g(e)}),arguments)};b.wbg.__wbg_isSafeInteger_d8c89788832a17bf=(a=>{const b=Number.isSafeInteger(c(a));return b});b.wbg.__wbg_entries_7a47f5716366056b=(a=>{const b=a2.entries(c(a));return g(b)});b.wbg.__wbg_is_a5728dbfb61c82cd=((a,b)=>{const d=a2.is(c(a),c(b));return d});b.wbg.__wbg_resolve_97ecd55ee839391b=(a=>{const b=Promise.resolve(c(a));return g(b)});b.wbg.__wbg_then_7aeb7c5f1536640f=((a,b)=>{const d=c(a).then(c(b));return g(d)});b.wbg.__wbg_then_5842e4e97f7beace=((a,b,d)=>{const e=c(a).then(c(b),c(d));return g(e)});b.wbg.__wbg_buffer_5d1b598a01b41a42=(a=>{const b=c(a).buffer;return g(b)});b.wbg.__wbg_newwithbyteoffsetandlength_d695c7957788f922=((a,b,d)=>{const e=new V(c(a),b>>>U,d>>>U);return g(e)});b.wbg.__wbg_new_ace717933ad7117f=(a=>{const b=new V(c(a));return g(b)});b.wbg.__wbg_set_74906aa30864df5a=((a,b,d)=>{c(a).set(c(b),d>>>U)});b.wbg.__wbg_length_f0764416ba5bb237=(a=>{const b=c(a).length;return b});b.wbg.__wbg_instanceof_Uint8Array_4f5cffed7df34b2f=(a=>{let b;try{b=c(a) instanceof V}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_newwithlength_728575f3bba9959b=(a=>{const b=new V(a>>>U);return g(b)});b.wbg.__wbg_subarray_7f7a652672800851=((a,b,d)=>{const e=c(a).subarray(b>>>U,d>>>U);return g(e)});b.wbg.__wbg_stringify_daa6661e90c04140=function(){return C((a=>{const b=$(c(a));return g(b)}),arguments)};b.wbg.__wbg_has_a2919659b7b645b3=function(){return C(((a,b)=>{const d=a1.has(c(a),c(b));return d}),arguments)};b.wbg.__wbg_set_37a50e901587b477=function(){return C(((a,b,d)=>{const e=a1.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=u(c(d));const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ Q]=g;p()[b/a0+ U]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new T(k(a,b))});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return g(b)});b.wbg.__wbindgen_closure_wrapper1962=((a,b,c)=>{const d=v(a,b,978,y);return g(d)});b.wbg.__wbindgen_closure_wrapper2102=((a,b,c)=>{const d=z(a,b,1025,A);return g(d)});b.wbg.__wbindgen_closure_wrapper2211=((a,b,c)=>{const d=z(a,b,1098,B);return g(d)});return b});var E=(()=>{if(D===P||D.byteLength===U){D=new Uint32Array(a.memory.buffer)};return D});var n=(()=>{if(m===P||m.byteLength===U){m=new Float64Array(a.memory.buffer)};return m});var v=((b,c,d,e)=>{const f={a:b,b:c,cnt:Q,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===U){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=U}}};g.original=f;return g});var e=(a=>{if(a<132)return;b[a]=d;d=a});var z=((b,c,d,e)=>{const f={a:b,b:c,cnt:Q,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=U;try{return e(c,f.b,...b)}finally{if(--f.cnt===U){a.__wbindgen_export_2.get(f.dtor)(c,f.b)}else{f.a=c}}};g.original=f;return g});var t=((a,b,c)=>{if(c===O){const c=r.encode(a);const d=b(c.length,Q)>>>U;j().subarray(d,d+ c.length).set(c);q=c.length;return d};let d=a.length;let e=b(d,Q)>>>U;const f=j();let g=U;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==U){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,Q)>>>U;const b=j().subarray(e+ g,e+ d);const f=s(a,b);g+=f.written};q=g;return e});var j=(()=>{if(i===P||i.byteLength===U){i=new V(a.memory.buffer)};return i});var k=((a,b)=>{a=a>>>U;return h.decode(j().subarray(a,a+ b))});var x=(a=>{if(w==Q)throw new T(`out of js stack`);b[--w]=a;return w});let a;const b=new M(N).fill(O);b.push(O,P,!0,!1);let d=b.length;const h=typeof TextDecoder!==R?new TextDecoder(S,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw T(`TextDecoder not available`)}};if(typeof TextDecoder!==R){h.decode()};let i=P;let m=P;let o=P;let q=U;const r=typeof TextEncoder!==R?new TextEncoder(S):{encode:()=>{throw T(`TextEncoder not available`)}};const s=typeof r.encodeInto===W?((a,b)=>r.encodeInto(a,b)):((a,b)=>{const c=r.encode(a);b.set(c);return {read:a.length,written:c.length}});let w=N;let D=P;export default L;export{K as initSync}