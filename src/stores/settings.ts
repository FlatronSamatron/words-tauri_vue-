import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Settings } from '../types'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<Settings>({
    interval_minutes: 5,
    direction: 'native_to_foreign'
  })

  async function fetchSettings() {
    const raw: any = await invoke('get_settings')
    settings.value = {
      interval_minutes: raw.interval_minutes,
      direction: raw.direction as 'native_to_foreign' | 'foreign_to_native'
    }
  }

  async function saveSettings(newSettings: Settings) {
    await invoke('save_settings', {
      intervalMinutes: newSettings.interval_minutes,
      direction: newSettings.direction
    })
    await invoke('update_timer_interval', { minutes: newSettings.interval_minutes })
    settings.value = { ...newSettings }
  }

  return { settings, fetchSettings, saveSettings }
})
