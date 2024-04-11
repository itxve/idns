import { invoke } from "@tauri-apps/api/tauri";

export async function get_dns<
  T = [string, { label: string; value: string }[]]
>() {
  return await invoke<T>("get_dns");
}
export async function set_dns(dns: string) {
  return await invoke("set_dns", { dns });
}
export async function add_dns(dns: string) {
  await invoke("add_dns", { dns });
}
export async function del_dns(dns: string) {
  return await invoke("del_dns", { dns });
}

export function refresh_tray_menu() {
  invoke("refresh_tray_menu");
}

export default { get_dns, set_dns, add_dns, del_dns, refresh_tray_menu };
