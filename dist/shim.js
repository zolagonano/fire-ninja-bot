// build/worker/shim.mjs
import D from "./2d9f49e1f79d77810dde910b9da335416ba18159-index.wasm";
import Ke from "./2d9f49e1f79d77810dde910b9da335416ba18159-index.wasm";
import { WorkerEntrypoint as Ne } from "cloudflare:workers";
var C = Object.defineProperty;
var B = (t, e) => {
  for (var n in e)
    C(t, n, { get: e[n], enumerable: true });
};
var d = {};
B(d, { IntoUnderlyingByteSource: () => A, IntoUnderlyingSink: () => E, IntoUnderlyingSource: () => q, MinifyConfig: () => T, PolishConfig: () => Z, R2Range: () => I, RequestRedirect: () => Y, __wbg_abort_2aa7521d5690750e: () => Et, __wbg_append_7bfcb4937d1d5e29: () => Dt, __wbg_buffer_12d079cc21e14bdb: () => ke, __wbg_buffer_dd7f74bc60f1faab: () => qe, __wbg_byobRequest_72fca99f9c32c193: () => qt, __wbg_byteLength_58f7b4fab1919d44: () => Te, __wbg_byteOffset_81d60f7392524f62: () => Ie, __wbg_call_27c0f87801dedf93: () => se, __wbg_call_b3ca7c6051f9bec1: () => he, __wbg_cause_3d9c85ebaf6b1155: () => pe, __wbg_cf_b1ddc6b9d2f719aa: () => Ft, __wbg_close_184931724d961ccc: () => Tt, __wbg_close_a994f9425dab445c: () => $t, __wbg_done_298b57d23c0fc80c: () => _e, __wbg_enqueue_ea194723156c0cc2: () => Nt, __wbg_error_8e3928cfb8a43e2b: () => Mt, __wbg_fetch_1db5b0ae726d68b5: () => wt, __wbg_fetch_921fad6ef9e883dd: () => kt, __wbg_get_e3c254076557e348: () => ce, __wbg_getwithrefkey_edc2c8960f0f1191: () => jt, __wbg_globalThis_d1e6af4856ba331b: () => ge, __wbg_global_207b558942527489: () => be, __wbg_has_0af94d20077affa2: () => ve, __wbg_headers_9620bfada380764a: () => Xt, __wbg_headers_abb199c3be8d817c: () => Lt, __wbg_instanceof_ArrayBuffer_836825be07d4c9d2: () => de, __wbg_instanceof_Error_e20bb56fd5591a93: () => le, __wbg_instanceof_Response_849eb93e75734b6e: () => Pt, __wbg_instanceof_Uint8Array_2b3bbecd033d19f6: () => Ae, __wbg_isSafeInteger_f7b04ef02296c4d2: () => xe, __wbg_iterator_2cee6dadfd956dfa: () => ie, __wbg_json_a4ff4ba48efc7bb8: () => Ct, __wbg_length_c20a40f15020d68a: () => ze, __wbg_method_83327ed2e3f3229c: () => It, __wbg_new_0d76b0581eca6298: () => At, __wbg_new_28c511d9baebfa89: () => we, __wbg_new_63b92bc8671ed464: () => Oe, __wbg_new_72fb9a18b5ae2624: () => ue, __wbg_new_81740750da40724f: () => me, __wbg_new_ab6fd82b10560829: () => Bt, __wbg_newnoargs_e258087cd0daa0ea: () => ee, __wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb: () => Me, __wbg_newwithlength_e9b4878cebadb3d3: () => Ee, __wbg_newwithoptbuffersourceandinit_a4fa81e77259bb96: () => Vt, __wbg_newwithoptreadablestreamandinit_0b825f969ca543d6: () => Kt, __wbg_newwithoptstrandinit_219732174c595a25: () => Gt, __wbg_newwithstrandinit_3fd6fba4083ff2d0: () => Ut, __wbg_next_196c84450b364254: () => re, __wbg_next_40fc327bfc8770e6: () => ne, __wbg_queueMicrotask_3cbae2ec6b6cd3d6: () => Zt, __wbg_queueMicrotask_481971b0d87f3dd4: () => Yt, __wbg_resolve_b0083a7967828ec8: () => Re, __wbg_respond_b1a43b2e3a06d525: () => St, __wbg_self_ce0dbfc45cf2f5be: () => fe, __wbg_set_1f9b04f170055d33: () => Le, __wbg_set_a47bac70306a19a7: () => Se, __wbg_set_cb0e7a5c2dd66afd: () => Wt, __wbg_signal_a61f78a3478fd9bc: () => zt, __wbg_status_61a01141acd3cf74: () => Jt, __wbg_stringify_8887fe74e1c50d81: () => Ue, __wbg_text_450a059667fd91fd: () => Qt, __wbg_then_0c86a60e8fcfe9f6: () => je, __wbg_then_a73caa9a87991566: () => Fe, __wbg_toString_ffe4c9ea3b3532e9: () => ye, __wbg_url_5f6dc4009ac5f99d: () => Ht, __wbg_url_7807f6a1fddc3e23: () => vt, __wbg_value_d93c65011f51a456: () => oe, __wbg_view_7f0ce470793a340f: () => Ot, __wbg_window_c6fb939a7f436783: () => ae, __wbindgen_as_number: () => Rt, __wbindgen_bigint_from_i64: () => bt, __wbindgen_bigint_get_as_i64: () => Ce, __wbindgen_boolean_get: () => mt, __wbindgen_cb_drop: () => st, __wbindgen_closure_wrapper988: () => $e, __wbindgen_debug_string: () => Be, __wbindgen_error_new: () => lt, __wbindgen_in: () => at, __wbindgen_is_bigint: () => gt, __wbindgen_is_function: () => te, __wbindgen_is_object: () => ut, __wbindgen_is_undefined: () => ft, __wbindgen_jsval_eq: () => dt, __wbindgen_jsval_loose_eq: () => xt, __wbindgen_memory: () => We, __wbindgen_number_get: () => yt, __wbindgen_number_new: () => ht, __wbindgen_object_clone_ref: () => pt, __wbindgen_object_drop_ref: () => ot, __wbindgen_string_get: () => ct, __wbindgen_string_new: () => it, __wbindgen_throw: () => De, fetch: () => v, getMemory: () => $ });
var W = new WebAssembly.Instance(D, { "./index_bg.js": d });
var o = W.exports;
function $() {
  return o.memory;
}
var w = new Array(128).fill(void 0);
w.push(void 0, null, true, false);
function r(t) {
  return w[t];
}
var h = w.length;
function N(t) {
  t < 132 || (w[t] = h, h = t);
}
function p(t) {
  let e = r(t);
  return N(t), e;
}
var P = typeof TextDecoder > "u" ? (0, module.require)("util").TextDecoder : TextDecoder;
var U = new P("utf-8", { ignoreBOM: true, fatal: true });
U.decode();
var R = null;
function M() {
  return (R === null || R.byteLength === 0) && (R = new Uint8Array(o.memory.buffer)), R;
}
function l(t, e) {
  return t = t >>> 0, U.decode(M().subarray(t, t + e));
}
function i(t) {
  h === w.length && w.push(w.length + 1);
  let e = h;
  return h = w[e], w[e] = t, e;
}
var y = 0;
var H = typeof TextEncoder > "u" ? (0, module.require)("util").TextEncoder : TextEncoder;
var O = new H("utf-8");
var J = typeof O.encodeInto == "function" ? function(t, e) {
  return O.encodeInto(t, e);
} : function(t, e) {
  let n = O.encode(t);
  return e.set(n), { read: t.length, written: n.length };
};
function x(t, e, n) {
  if (n === void 0) {
    let g = O.encode(t), m = e(g.length, 1) >>> 0;
    return M().subarray(m, m + g.length).set(g), y = g.length, m;
  }
  let _ = t.length, c = e(_, 1) >>> 0, a = M(), u = 0;
  for (; u < _; u++) {
    let g = t.charCodeAt(u);
    if (g > 127)
      break;
    a[c + u] = g;
  }
  if (u !== _) {
    u !== 0 && (t = t.slice(u)), c = n(c, _, _ = u + t.length * 3, 1) >>> 0;
    let g = M().subarray(c + u, c + _);
    u += J(t, g).written, c = n(c, _, u, 1) >>> 0;
  }
  return y = u, c;
}
function b(t) {
  return t == null;
}
var j = null;
function f() {
  return (j === null || j.byteLength === 0) && (j = new Int32Array(o.memory.buffer)), j;
}
var F = null;
function X() {
  return (F === null || F.byteLength === 0) && (F = new Float64Array(o.memory.buffer)), F;
}
var k = null;
function V() {
  return (k === null || k.byteLength === 0) && (k = new BigInt64Array(o.memory.buffer)), k;
}
function z(t) {
  let e = typeof t;
  if (e == "number" || e == "boolean" || t == null)
    return `${t}`;
  if (e == "string")
    return `"${t}"`;
  if (e == "symbol") {
    let c = t.description;
    return c == null ? "Symbol" : `Symbol(${c})`;
  }
  if (e == "function") {
    let c = t.name;
    return typeof c == "string" && c.length > 0 ? `Function(${c})` : "Function";
  }
  if (Array.isArray(t)) {
    let c = t.length, a = "[";
    c > 0 && (a += z(t[0]));
    for (let u = 1; u < c; u++)
      a += ", " + z(t[u]);
    return a += "]", a;
  }
  let n = /\[object ([^\]]+)\]/.exec(toString.call(t)), _;
  if (n.length > 1)
    _ = n[1];
  else
    return toString.call(t);
  if (_ == "Object")
    try {
      return "Object(" + JSON.stringify(t) + ")";
    } catch {
      return "Object";
    }
  return t instanceof Error ? `${t.name}: ${t.message}
${t.stack}` : _;
}
var L = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((t) => {
  o.__wbindgen_export_2.get(t.dtor)(t.a, t.b);
});
function G(t, e, n, _) {
  let c = { a: t, b: e, cnt: 1, dtor: n }, a = (...u) => {
    c.cnt++;
    let g = c.a;
    c.a = 0;
    try {
      return _(g, c.b, ...u);
    } finally {
      --c.cnt === 0 ? (o.__wbindgen_export_2.get(c.dtor)(g, c.b), L.unregister(c)) : c.a = g;
    }
  };
  return a.original = c, L.register(a, c, c), a;
}
function K(t, e, n) {
  o._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4aab2945efae0c3b(t, e, i(n));
}
function v(t, e, n) {
  let _ = o.fetch(i(t), i(e), i(n));
  return p(_);
}
function s(t, e) {
  try {
    return t.apply(this, e);
  } catch (n) {
    o.__wbindgen_exn_store(i(n));
  }
}
function Q(t, e, n, _) {
  o.wasm_bindgen__convert__closures__invoke2_mut__h8709a2fe22c0b131(t, e, i(n), i(_));
}
var Y = Object.freeze({ Error: 0, 0: "Error", Follow: 1, 1: "Follow", Manual: 2, 2: "Manual" });
var Z = Object.freeze({ Off: 0, 0: "Off", Lossy: 1, 1: "Lossy", Lossless: 2, 2: "Lossless" });
var tt = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((t) => o.__wbg_intounderlyingbytesource_free(t >>> 0));
var A = class {
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, tt.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    o.__wbg_intounderlyingbytesource_free(e);
  }
  get type() {
    let e, n;
    try {
      let a = o.__wbindgen_add_to_stack_pointer(-16);
      o.intounderlyingbytesource_type(a, this.__wbg_ptr);
      var _ = f()[a / 4 + 0], c = f()[a / 4 + 1];
      return e = _, n = c, l(_, c);
    } finally {
      o.__wbindgen_add_to_stack_pointer(16), o.__wbindgen_free(e, n, 1);
    }
  }
  get autoAllocateChunkSize() {
    return o.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr) >>> 0;
  }
  start(e) {
    o.intounderlyingbytesource_start(this.__wbg_ptr, i(e));
  }
  pull(e) {
    let n = o.intounderlyingbytesource_pull(this.__wbg_ptr, i(e));
    return p(n);
  }
  cancel() {
    let e = this.__destroy_into_raw();
    o.intounderlyingbytesource_cancel(e);
  }
};
var et = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((t) => o.__wbg_intounderlyingsink_free(t >>> 0));
var E = class {
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, et.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    o.__wbg_intounderlyingsink_free(e);
  }
  write(e) {
    let n = o.intounderlyingsink_write(this.__wbg_ptr, i(e));
    return p(n);
  }
  close() {
    let e = this.__destroy_into_raw(), n = o.intounderlyingsink_close(e);
    return p(n);
  }
  abort(e) {
    let n = this.__destroy_into_raw(), _ = o.intounderlyingsink_abort(n, i(e));
    return p(_);
  }
};
var nt = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((t) => o.__wbg_intounderlyingsource_free(t >>> 0));
var q = class {
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, nt.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    o.__wbg_intounderlyingsource_free(e);
  }
  pull(e) {
    let n = o.intounderlyingsource_pull(this.__wbg_ptr, i(e));
    return p(n);
  }
  cancel() {
    let e = this.__destroy_into_raw();
    o.intounderlyingsource_cancel(e);
  }
};
var rt = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((t) => o.__wbg_minifyconfig_free(t >>> 0));
var T = class {
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, rt.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    o.__wbg_minifyconfig_free(e);
  }
  get js() {
    return o.__wbg_get_minifyconfig_js(this.__wbg_ptr) !== 0;
  }
  set js(e) {
    o.__wbg_set_minifyconfig_js(this.__wbg_ptr, e);
  }
  get html() {
    return o.__wbg_get_minifyconfig_html(this.__wbg_ptr) !== 0;
  }
  set html(e) {
    o.__wbg_set_minifyconfig_html(this.__wbg_ptr, e);
  }
  get css() {
    return o.__wbg_get_minifyconfig_css(this.__wbg_ptr) !== 0;
  }
  set css(e) {
    o.__wbg_set_minifyconfig_css(this.__wbg_ptr, e);
  }
};
var _t = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((t) => o.__wbg_r2range_free(t >>> 0));
var I = class {
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, _t.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    o.__wbg_r2range_free(e);
  }
  get offset() {
    try {
      let _ = o.__wbindgen_add_to_stack_pointer(-16);
      o.__wbg_get_r2range_offset(_, this.__wbg_ptr);
      var e = f()[_ / 4 + 0], n = f()[_ / 4 + 1];
      return e === 0 ? void 0 : n >>> 0;
    } finally {
      o.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set offset(e) {
    o.__wbg_set_r2range_offset(this.__wbg_ptr, !b(e), b(e) ? 0 : e);
  }
  get length() {
    try {
      let _ = o.__wbindgen_add_to_stack_pointer(-16);
      o.__wbg_get_r2range_length(_, this.__wbg_ptr);
      var e = f()[_ / 4 + 0], n = f()[_ / 4 + 1];
      return e === 0 ? void 0 : n >>> 0;
    } finally {
      o.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set length(e) {
    o.__wbg_set_r2range_length(this.__wbg_ptr, !b(e), b(e) ? 0 : e);
  }
  get suffix() {
    try {
      let _ = o.__wbindgen_add_to_stack_pointer(-16);
      o.__wbg_get_r2range_suffix(_, this.__wbg_ptr);
      var e = f()[_ / 4 + 0], n = f()[_ / 4 + 1];
      return e === 0 ? void 0 : n >>> 0;
    } finally {
      o.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set suffix(e) {
    o.__wbg_set_r2range_suffix(this.__wbg_ptr, !b(e), b(e) ? 0 : e);
  }
};
function ot(t) {
  p(t);
}
function it(t, e) {
  let n = l(t, e);
  return i(n);
}
function ct(t, e) {
  let n = r(e), _ = typeof n == "string" ? n : void 0;
  var c = b(_) ? 0 : x(_, o.__wbindgen_malloc, o.__wbindgen_realloc), a = y;
  f()[t / 4 + 1] = a, f()[t / 4 + 0] = c;
}
function st(t) {
  let e = p(t).original;
  return e.cnt-- == 1 ? (e.a = 0, true) : false;
}
function ut(t) {
  let e = r(t);
  return typeof e == "object" && e !== null;
}
function ft(t) {
  return r(t) === void 0;
}
function at(t, e) {
  return r(t) in r(e);
}
function gt(t) {
  return typeof r(t) == "bigint";
}
function bt(t) {
  return i(t);
}
function dt(t, e) {
  return r(t) === r(e);
}
function lt(t, e) {
  let n = new Error(l(t, e));
  return i(n);
}
function wt(t) {
  let e = fetch(r(t));
  return i(e);
}
function pt(t) {
  let e = r(t);
  return i(e);
}
function yt(t, e) {
  let n = r(e), _ = typeof n == "number" ? n : void 0;
  X()[t / 8 + 1] = b(_) ? 0 : _, f()[t / 4 + 0] = !b(_);
}
function ht(t) {
  return i(t);
}
function xt(t, e) {
  return r(t) == r(e);
}
function mt(t) {
  let e = r(t);
  return typeof e == "boolean" ? e ? 1 : 0 : 2;
}
function Rt(t) {
  return +r(t);
}
function jt(t, e) {
  let n = r(t)[r(e)];
  return i(n);
}
function Ft() {
  return s(function(t) {
    let e = r(t).cf;
    return b(e) ? 0 : i(e);
  }, arguments);
}
function kt(t, e) {
  let n = r(t).fetch(r(e));
  return i(n);
}
function Mt(t) {
  console.error(r(t));
}
function Ot(t) {
  let e = r(t).view;
  return b(e) ? 0 : i(e);
}
function St() {
  return s(function(t, e) {
    r(t).respond(e >>> 0);
  }, arguments);
}
function zt(t) {
  let e = r(t).signal;
  return i(e);
}
function At() {
  return s(function() {
    let t = new AbortController();
    return i(t);
  }, arguments);
}
function Et(t) {
  r(t).abort();
}
function qt(t) {
  let e = r(t).byobRequest;
  return b(e) ? 0 : i(e);
}
function Tt() {
  return s(function(t) {
    r(t).close();
  }, arguments);
}
function It(t, e) {
  let n = r(e).method, _ = x(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = y;
  f()[t / 4 + 1] = c, f()[t / 4 + 0] = _;
}
function vt(t, e) {
  let n = r(e).url, _ = x(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = y;
  f()[t / 4 + 1] = c, f()[t / 4 + 0] = _;
}
function Lt(t) {
  let e = r(t).headers;
  return i(e);
}
function Ut() {
  return s(function(t, e, n) {
    let _ = new Request(l(t, e), r(n));
    return i(_);
  }, arguments);
}
function Ct() {
  return s(function(t) {
    let e = r(t).json();
    return i(e);
  }, arguments);
}
function Bt() {
  return s(function() {
    let t = new Headers();
    return i(t);
  }, arguments);
}
function Dt() {
  return s(function(t, e, n, _, c) {
    r(t).append(l(e, n), l(_, c));
  }, arguments);
}
function Wt() {
  return s(function(t, e, n, _, c) {
    r(t).set(l(e, n), l(_, c));
  }, arguments);
}
function $t() {
  return s(function(t) {
    r(t).close();
  }, arguments);
}
function Nt() {
  return s(function(t, e) {
    r(t).enqueue(r(e));
  }, arguments);
}
function Pt(t) {
  let e;
  try {
    e = r(t) instanceof Response;
  } catch {
    e = false;
  }
  return e;
}
function Ht(t, e) {
  let n = r(e).url, _ = x(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = y;
  f()[t / 4 + 1] = c, f()[t / 4 + 0] = _;
}
function Jt(t) {
  return r(t).status;
}
function Xt(t) {
  let e = r(t).headers;
  return i(e);
}
function Vt() {
  return s(function(t, e) {
    let n = new Response(r(t), r(e));
    return i(n);
  }, arguments);
}
function Gt() {
  return s(function(t, e, n) {
    let _ = new Response(t === 0 ? void 0 : l(t, e), r(n));
    return i(_);
  }, arguments);
}
function Kt() {
  return s(function(t, e) {
    let n = new Response(r(t), r(e));
    return i(n);
  }, arguments);
}
function Qt() {
  return s(function(t) {
    let e = r(t).text();
    return i(e);
  }, arguments);
}
function Yt(t) {
  queueMicrotask(r(t));
}
function Zt(t) {
  let e = r(t).queueMicrotask;
  return i(e);
}
function te(t) {
  return typeof r(t) == "function";
}
function ee(t, e) {
  let n = new Function(l(t, e));
  return i(n);
}
function ne(t) {
  let e = r(t).next;
  return i(e);
}
function re() {
  return s(function(t) {
    let e = r(t).next();
    return i(e);
  }, arguments);
}
function _e(t) {
  return r(t).done;
}
function oe(t) {
  let e = r(t).value;
  return i(e);
}
function ie() {
  return i(Symbol.iterator);
}
function ce() {
  return s(function(t, e) {
    let n = Reflect.get(r(t), r(e));
    return i(n);
  }, arguments);
}
function se() {
  return s(function(t, e) {
    let n = r(t).call(r(e));
    return i(n);
  }, arguments);
}
function ue() {
  let t = new Object();
  return i(t);
}
function fe() {
  return s(function() {
    let t = self.self;
    return i(t);
  }, arguments);
}
function ae() {
  return s(function() {
    let t = window.window;
    return i(t);
  }, arguments);
}
function ge() {
  return s(function() {
    let t = globalThis.globalThis;
    return i(t);
  }, arguments);
}
function be() {
  return s(function() {
    let t = global.global;
    return i(t);
  }, arguments);
}
function de(t) {
  let e;
  try {
    e = r(t) instanceof ArrayBuffer;
  } catch {
    e = false;
  }
  return e;
}
function le(t) {
  let e;
  try {
    e = r(t) instanceof Error;
  } catch {
    e = false;
  }
  return e;
}
function we(t, e) {
  let n = new Error(l(t, e));
  return i(n);
}
function pe(t) {
  let e = r(t).cause;
  return i(e);
}
function ye(t) {
  let e = r(t).toString();
  return i(e);
}
function he() {
  return s(function(t, e, n) {
    let _ = r(t).call(r(e), r(n));
    return i(_);
  }, arguments);
}
function xe(t) {
  return Number.isSafeInteger(r(t));
}
function me(t, e) {
  try {
    var n = { a: t, b: e }, _ = (a, u) => {
      let g = n.a;
      n.a = 0;
      try {
        return Q(g, n.b, a, u);
      } finally {
        n.a = g;
      }
    };
    let c = new Promise(_);
    return i(c);
  } finally {
    n.a = n.b = 0;
  }
}
function Re(t) {
  let e = Promise.resolve(r(t));
  return i(e);
}
function je(t, e) {
  let n = r(t).then(r(e));
  return i(n);
}
function Fe(t, e, n) {
  let _ = r(t).then(r(e), r(n));
  return i(_);
}
function ke(t) {
  let e = r(t).buffer;
  return i(e);
}
function Me(t, e, n) {
  let _ = new Uint8Array(r(t), e >>> 0, n >>> 0);
  return i(_);
}
function Oe(t) {
  let e = new Uint8Array(r(t));
  return i(e);
}
function Se(t, e, n) {
  r(t).set(r(e), n >>> 0);
}
function ze(t) {
  return r(t).length;
}
function Ae(t) {
  let e;
  try {
    e = r(t) instanceof Uint8Array;
  } catch {
    e = false;
  }
  return e;
}
function Ee(t) {
  let e = new Uint8Array(t >>> 0);
  return i(e);
}
function qe(t) {
  let e = r(t).buffer;
  return i(e);
}
function Te(t) {
  return r(t).byteLength;
}
function Ie(t) {
  return r(t).byteOffset;
}
function ve() {
  return s(function(t, e) {
    return Reflect.has(r(t), r(e));
  }, arguments);
}
function Le() {
  return s(function(t, e, n) {
    return Reflect.set(r(t), r(e), r(n));
  }, arguments);
}
function Ue() {
  return s(function(t) {
    let e = JSON.stringify(r(t));
    return i(e);
  }, arguments);
}
function Ce(t, e) {
  let n = r(e), _ = typeof n == "bigint" ? n : void 0;
  V()[t / 8 + 1] = b(_) ? BigInt(0) : _, f()[t / 4 + 0] = !b(_);
}
function Be(t, e) {
  let n = z(r(e)), _ = x(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = y;
  f()[t / 4 + 1] = c, f()[t / 4 + 0] = _;
}
function De(t, e) {
  throw new Error(l(t, e));
}
function We() {
  let t = o.memory;
  return i(t);
}
function $e(t, e, n) {
  let _ = G(t, e, 222, K);
  return i(_);
}
var S = class extends Ne {
  async fetch(e) {
    return await v(e, this.env, this.ctx);
  }
  async queue(e) {
    return await (void 0)(e, this.env, this.ctx);
  }
  async scheduled(e) {
    return await (void 0)(e, this.env, this.ctx);
  }
};
var Pe = ["IntoUnderlyingByteSource", "IntoUnderlyingSink", "IntoUnderlyingSource", "MinifyConfig", "PolishConfig", "R2Range", "RequestRedirect", "fetch", "queue", "scheduled", "getMemory"];
Object.keys(d).map((t) => {
  Pe.includes(t) | t.startsWith("__") || (S.prototype[t] = d[t]);
});
var Ye = S;
export {
  A as IntoUnderlyingByteSource,
  E as IntoUnderlyingSink,
  q as IntoUnderlyingSource,
  T as MinifyConfig,
  Z as PolishConfig,
  I as R2Range,
  Y as RequestRedirect,
  Et as __wbg_abort_2aa7521d5690750e,
  Dt as __wbg_append_7bfcb4937d1d5e29,
  ke as __wbg_buffer_12d079cc21e14bdb,
  qe as __wbg_buffer_dd7f74bc60f1faab,
  qt as __wbg_byobRequest_72fca99f9c32c193,
  Te as __wbg_byteLength_58f7b4fab1919d44,
  Ie as __wbg_byteOffset_81d60f7392524f62,
  se as __wbg_call_27c0f87801dedf93,
  he as __wbg_call_b3ca7c6051f9bec1,
  pe as __wbg_cause_3d9c85ebaf6b1155,
  Ft as __wbg_cf_b1ddc6b9d2f719aa,
  Tt as __wbg_close_184931724d961ccc,
  $t as __wbg_close_a994f9425dab445c,
  _e as __wbg_done_298b57d23c0fc80c,
  Nt as __wbg_enqueue_ea194723156c0cc2,
  Mt as __wbg_error_8e3928cfb8a43e2b,
  wt as __wbg_fetch_1db5b0ae726d68b5,
  kt as __wbg_fetch_921fad6ef9e883dd,
  ce as __wbg_get_e3c254076557e348,
  jt as __wbg_getwithrefkey_edc2c8960f0f1191,
  ge as __wbg_globalThis_d1e6af4856ba331b,
  be as __wbg_global_207b558942527489,
  ve as __wbg_has_0af94d20077affa2,
  Xt as __wbg_headers_9620bfada380764a,
  Lt as __wbg_headers_abb199c3be8d817c,
  de as __wbg_instanceof_ArrayBuffer_836825be07d4c9d2,
  le as __wbg_instanceof_Error_e20bb56fd5591a93,
  Pt as __wbg_instanceof_Response_849eb93e75734b6e,
  Ae as __wbg_instanceof_Uint8Array_2b3bbecd033d19f6,
  xe as __wbg_isSafeInteger_f7b04ef02296c4d2,
  ie as __wbg_iterator_2cee6dadfd956dfa,
  Ct as __wbg_json_a4ff4ba48efc7bb8,
  ze as __wbg_length_c20a40f15020d68a,
  It as __wbg_method_83327ed2e3f3229c,
  At as __wbg_new_0d76b0581eca6298,
  we as __wbg_new_28c511d9baebfa89,
  Oe as __wbg_new_63b92bc8671ed464,
  ue as __wbg_new_72fb9a18b5ae2624,
  me as __wbg_new_81740750da40724f,
  Bt as __wbg_new_ab6fd82b10560829,
  ee as __wbg_newnoargs_e258087cd0daa0ea,
  Me as __wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb,
  Ee as __wbg_newwithlength_e9b4878cebadb3d3,
  Vt as __wbg_newwithoptbuffersourceandinit_a4fa81e77259bb96,
  Kt as __wbg_newwithoptreadablestreamandinit_0b825f969ca543d6,
  Gt as __wbg_newwithoptstrandinit_219732174c595a25,
  Ut as __wbg_newwithstrandinit_3fd6fba4083ff2d0,
  re as __wbg_next_196c84450b364254,
  ne as __wbg_next_40fc327bfc8770e6,
  Zt as __wbg_queueMicrotask_3cbae2ec6b6cd3d6,
  Yt as __wbg_queueMicrotask_481971b0d87f3dd4,
  Re as __wbg_resolve_b0083a7967828ec8,
  St as __wbg_respond_b1a43b2e3a06d525,
  fe as __wbg_self_ce0dbfc45cf2f5be,
  Le as __wbg_set_1f9b04f170055d33,
  Se as __wbg_set_a47bac70306a19a7,
  Wt as __wbg_set_cb0e7a5c2dd66afd,
  zt as __wbg_signal_a61f78a3478fd9bc,
  Jt as __wbg_status_61a01141acd3cf74,
  Ue as __wbg_stringify_8887fe74e1c50d81,
  Qt as __wbg_text_450a059667fd91fd,
  je as __wbg_then_0c86a60e8fcfe9f6,
  Fe as __wbg_then_a73caa9a87991566,
  ye as __wbg_toString_ffe4c9ea3b3532e9,
  Ht as __wbg_url_5f6dc4009ac5f99d,
  vt as __wbg_url_7807f6a1fddc3e23,
  oe as __wbg_value_d93c65011f51a456,
  Ot as __wbg_view_7f0ce470793a340f,
  ae as __wbg_window_c6fb939a7f436783,
  Rt as __wbindgen_as_number,
  bt as __wbindgen_bigint_from_i64,
  Ce as __wbindgen_bigint_get_as_i64,
  mt as __wbindgen_boolean_get,
  st as __wbindgen_cb_drop,
  $e as __wbindgen_closure_wrapper988,
  Be as __wbindgen_debug_string,
  lt as __wbindgen_error_new,
  at as __wbindgen_in,
  gt as __wbindgen_is_bigint,
  te as __wbindgen_is_function,
  ut as __wbindgen_is_object,
  ft as __wbindgen_is_undefined,
  dt as __wbindgen_jsval_eq,
  xt as __wbindgen_jsval_loose_eq,
  We as __wbindgen_memory,
  yt as __wbindgen_number_get,
  ht as __wbindgen_number_new,
  pt as __wbindgen_object_clone_ref,
  ot as __wbindgen_object_drop_ref,
  ct as __wbindgen_string_get,
  it as __wbindgen_string_new,
  De as __wbindgen_throw,
  Ye as default,
  v as fetch,
  $ as getMemory,
  Ke as wasmModule
};
//# sourceMappingURL=shim.js.map
