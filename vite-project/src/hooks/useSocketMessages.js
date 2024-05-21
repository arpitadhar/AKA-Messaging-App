import { useEffect, useRef } from 'react';
import { io } from 'socket.io-client';
import useConversation from './useConversation';


/*
  initially wanted all socketio related things to be handled by this hook but it complicated things
*/
const useSocketMessages = (setMessages) => {
  const { selectedConversation } = useConversation();
  const socketRef = useRef(null);

  useEffect(() => {
    if (selectedConversation?.id) {
      socketRef.current = io("http://localhost:3000");

      socketRef.current.emit('join', selectedConversation.id);

      socketRef.current.on('message', (message) => {
        console.log("useSocketMessage.js: ", message);
        
        setMessages((prevMessages) => [...prevMessages, message]);
      });

      return () => {
        if (socketRef.current) {
          socketRef.current.emit('leave', selectedConversation.id);
          socketRef.current.disconnect();
          socketRef.current = null;
        }
      };
    }
  }, [selectedConversation?.id, setMessages]);

  return socketRef.current;
};

export default useSocketMessages;
