import { defineStore } from 'pinia'

export const useUserStore = defineStore('user', {
  state: () => ({
    username: '',
    email: '',
    isLoggedIn: false,
  }),
  actions: {
    login({ username, email }) {
      this.username = username
      this.email = email
      this.isLoggedIn = true
    },
    logout() {
      this.username = ''
      this.email = ''
      this.isLoggedIn = false
    }
  }
})