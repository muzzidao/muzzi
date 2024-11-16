import { create } from 'zustand'
import { NewMessage, MuzziPlayers } from '../types/MuzziPlayer'
import { persist, createJSONStorage } from 'zustand/middleware'

export interface MuzziPlayerStore {
  muzzi_players: MuzziPlayers
  addMessage: (msg: NewMessage) => void
  get: () => MuzziPlayerStore
  set: (partial: MuzziPlayerStore | Partial<MuzziPlayerStore>) => void
}

const useMuzziPlayerStore = create<MuzziPlayerStore>()(
  persist(
    (set, get) => ({
      muzzi_players: { "New MuzziPlayer": [] },
      addMessage: (msg: NewMessage) => {
        const { muzzi_players } = get()
        const { muzzi_player, author, content } = msg
        if (!muzzi_players[muzzi_player]) {
          muzzi_players[muzzi_player] = []
        }
        muzzi_players[muzzi_player].push({ author, content })
        set({ muzzi_players })
      },

      get,
      set,
    }),
    {
      name: 'muzzi_player', // unique name
      storage: createJSONStorage(() => sessionStorage), // (optional) by default, 'localStorage' is used
    }
  )
)

export default useMuzziPlayerStore
