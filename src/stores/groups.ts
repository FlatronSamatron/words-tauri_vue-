import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Group } from '../types'

export const useGroupsStore = defineStore('groups', () => {
  const groups = ref<Group[]>([])

  async function fetchGroups() {
    const res: Group[] = await invoke('get_groups')
    groups.value = res
  }

  async function addGroup(name: string) {
    const res: Group = await invoke('add_group', { name })
    groups.value.push(res)
    return res
  }

  async function renameGroup(id: number, name: string) {
    const res: Group = await invoke('rename_group', { id, name })
    const index = groups.value.findIndex(g => g.id === id)
    if (index !== -1) {
      groups.value[index] = res
    }
    return res
  }

  async function deleteGroup(id: number) {
    const success: boolean = await invoke('delete_group', { id })
    if (success) {
      groups.value = groups.value.filter(g => g.id !== id)
    }
    return success
  }

  return { groups, fetchGroups, addGroup, renameGroup, deleteGroup }
})
