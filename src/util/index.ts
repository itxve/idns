export async function V2Str(query) {
  for (let [key, value] of Object.entries(query)) {
    if (!value) {
      continue;
    }
    if (typeof value == "object") {
      query[key] = Array.isArray(value) ? value.toString() : V2Str(value);
    } else if (typeof value != "string") {
      query[key] = String(value);
    }
  }
  return query;
}

export function sleep(ms: number) {
  return new Promise((fn) => setTimeout(fn, ms));
}

// 节流时间戳版本
export function throttle(func, wait) {
  let previous = 0;
  return function (this) {
    let now = Date.now(),
      context = this,
      args = [...arguments];
    if (now - previous > wait) {
      func.apply(context, args);
      previous = now; // 闭包，记录本次执行时间戳
    }
  };
}
