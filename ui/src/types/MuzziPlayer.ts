export interface MuzziPlayerMessage {
  author: string
  content: string
}

export interface NewMessage {
  muzzi_player: string
  author: string
  content: string
}

export interface SendMuzziPlayerMessage {
  Send: {
    target: string
    message: string
  }
}

// MuzziPlayers consists of a map of counterparty to an array of messages
export interface MuzziPlayers {
  [counterparty: string]: MuzziPlayerMessage[]
}
