import { create } from 'zustand';

const useConversation = create((set) => ({
  socket: null,
  setSocket: (socket) => set({ socket }),

  selectedConversation: null,
  setSelectedConversation: (conversation) => set((state) => {
    
    state.socket.emit('join', conversation);
    return { selectedConversation: conversation };
  }),

  messages: [],
  setMessages: (messages) => set({ messages }),
}));

export default useConversation;
