import { useState } from 'react'
import useConversation from './useConversation'
import toast from 'react-hot-toast'; 

const useSendMessage = () => {
  const [loading, setLoading] = useState(false);
  const {messages, setMessages, selectedConversation} = useConversation(); 
  const senderUser = sessionStorage.getItem("email");
  const senderName = senderUser.replace(/^"(.*)"$/, '$1');
  const sendMessage = async(messageBody) => {
   setLoading(true)
   try{
       const res = await fetch("http://localhost:3000/create-message", {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json'
        },
        body:JSON.stringify({username: senderName, message: messageBody, conversation_id: selectedConversation.id})
      });
      const data = await res.json()
      if(data.error) throw new Error(data.error)
      setMessages([...messages,data]);
   }
    catch(error){
    toast.error(error.message); 
   }finally{
    setLoading(false)
   }
};
return { sendMessage, loading }
}; 

export default useSendMessage; 