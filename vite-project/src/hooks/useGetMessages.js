import { useEffect } from 'react';
import { useState } from 'react'; 
import useConversation from './useConversation';
import useSendMessage from './useSendMessage';
import toast from 'react-hot-toast';

const useGetMessages = () => {
  const [loading, setLoading] = useState(false)
  const {messages, setMessages, selectedConversation} = useConversation()

  useEffect(() => {
      const getMessages = async () => {
        setLoading(true)
        try{
            const res = await fetch("http://localhost:3000/messages", {
                method: "POST", 
                headers: {
                    "Content-Type": "application/json"
                }, 
                body: JSON.stringify(selectedConversation.id)
            })
            const data = await res.json(); 
            console.log(selectedConversation.id); 
            if(data.error) throw new Error(data.error);
            setMessages(transformMessages(data));
            //setMessages(data); 
            }
            catch (error) {
              toast.error(error.message);
            } finally {
              setLoading(false);
            }
          }
      if(selectedConversation?.id) getMessages();
  }, [selectedConversation?.id, setMessages]);
  return {messages, loading};
}
export default useGetMessages;

const transformMessages = (data) => {
    if (Array.isArray(data)) {
        return data; 
    } else if (typeof data === 'string') {
        return JSON.parse(data);
    } else if (data &&  typeof data === 'object') {
        return [data]; 
    } else {
        throw new Error("Invalid data format received.");
    }
};