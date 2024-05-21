import { useState } from 'react';
import useConversation from './useConversation';
import toast from 'react-hot-toast';
import { io } from 'socket.io-client';

const useSendMessage = (socket) => {
  const [loading, setLoading] = useState(false);
  const { messages, setMessages, selectedConversation } = useConversation();
  const senderUser = sessionStorage.getItem("email");
  const senderName = senderUser.replace(/^"(.*)"$/, '$1');

  const sendMessage = async (messageBody) => {
    setLoading(true);
    try {
      const res = await fetch("http://localhost:3000/create-message", {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ user_id: senderName, message: messageBody, conversation_id: selectedConversation.id })
      });
      const data = await res.json();
      if (data.error) throw new Error(data.error);

      socket.emit("message", {
        text: messageBody,
        room: selectedConversation.id,
        user: senderName
      });
      console.log(socket.id);
      setMessages([...messages, messageBody]);
   
    } catch (error) {
      toast.error(error.message);
    } finally {
      setLoading(false);
    }
  };

  return { sendMessage, loading };
};

export default useSendMessage;
