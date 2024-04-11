<template>
  <div class="dns">
    <div>
      <NFormItem :rule="rule">
        <NSpace :inline="true">
          <span style="height: 24px;" slot="label">dns:<b>{{ selectRef }}</b></span>
          <n-input v-model:value="inputValue" aria-autocomplete="none" placeholder="dns like 192.10.1.1" />
          <NButton :disabled="!isValidDNS(inputValue) || dns_exsit(inputValue)" @click="add_dns">添加</NButton>
        </NSpace>
      </NFormItem>
    </div>
    <NDataTable :columns="columns" :data="dataRef" :pagination="{
      pageSize: 8,
    }" />
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, h } from "vue";
import {
  NButton,
  NSpace,
  NDataTable,
  NInput,
  NFormItem,
  FormItemRule,
} from "naive-ui";
import cmds from "@/plugins/cmds"


const columns = [
  {
    title: "序号",
    key: "no",
  },
  {
    title: "dns",
    key: "value",
  },
  {
    title: "操作",
    key: "actions",
    render(row: { label: string; value: string }) {
      return h(
        NSpace, [h(
          NButton,
          {
            strong: true,
            tertiary: true,
            size: "small",
            onClick: () => { set_dns(row.value) },
          },
          { default: () => "set" }
        ), h(
          NButton,
          {
            strong: true,
            tertiary: true,
            type: "error",
            size: "small",
            onClick: () => { del_dns(row.value) },
          },
          { default: () => "del" }
        )]
      );
    },
  },
];

const selectRef = ref("");
const dataRef = ref<{ label: string; value: string }[]>([]);
const inputValue = ref("");

const loadDns = async () => {
  const [select, dns] = await cmds.get_dns();
  selectRef.value = select;
  dataRef.value = dns.map((v, i) => {
    v["no"] = i + 1;
    return v;
  });

  await cmds.refresh_tray_menu();
};


const dns_exsit = (dns) => {
  return !!dataRef.value?.filter(it => it.value === dns).length
}

const add_dns = async () => {
  await cmds.add_dns(inputValue.value)
  await loadDns()
  inputValue.value = ""
};

const del_dns = async (dns) => {
  await cmds.del_dns(dns)
  await loadDns()
};

const set_dns = async (dns) => {
  await cmds.set_dns(dns)
  await loadDns()
};


const isValidDNS = (dns) => {
  const dnsOrIPRegex =
    /^(?![0-9]+$)(?!.*--)([a-zA-Z0-9-]+(?<!-)\.)+[a-zA-Z]{2,}|(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;
  return dnsOrIPRegex.test(dns);
};

const rule: FormItemRule = {
  trigger: ["input", "blur"],
  validator: function () {
    if (isValidDNS(inputValue.value)) {
      return dns_exsit(inputValue.value) ? new Error("dns 已存在") : true
    }
    return new Error("dns 格式错误");
  },
};
onMounted(async () => {
  loadDns();
});
</script>

<style scoped>
.dns {
  padding: 0 20px 0 20px;
}
</style>
