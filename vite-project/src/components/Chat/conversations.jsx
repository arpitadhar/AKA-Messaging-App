import { useState, useEffect, useRef, forwardRef, useImperativeHandle } from "react";
import Conversation from "./conversation"; 
import toast from 'react-hot-toast'; 
import { Flex, Spin } from "antd";
const Conversations = forwardRef(({socket},ref) => {
    const [loading, setLoading] = useState(false);
    const [conversations, setConversations] = useState([]); 
    const usernameDefault = sessionStorage.getItem("email");
    const user = usernameDefault.replace(/^"(.*)"$/, '$1');

        const getConversations = async () => {
            setLoading(true); 
            try {
                const payload = { user};
                console.log("Sending payload:", payload);

                const res = await fetch("http://localhost:3000/list-conversations", {
                    method: 'POST', 
                    headers: {
                        'Content-Type': 'application/json'
                    }, 
                    body: JSON.stringify(user)
                }); 

                if (!res.ok) {
                    const errorText = await res.text();
                    throw new Error(`Error ${res.status}: ${errorText}`);
                }

                const data = await res.json(); 
                if (data.error) {
                    throw new Error(data.error); 
                }
                setConversations(data); 
            } catch (error) {
                toast.error(error.message); 
            } finally {
                setLoading(false); 
            }
    }; 
    
    useImperativeHandle(ref, () => ({
        getConversations
    }));

    useEffect(() => {
        getConversations();
    }, [user]); 

    const transformConversations = (data) => {
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
    console.log("conversations", transformConversations(conversations)); 
 
    return(
        <div className='py-2 flex flex-col overflow-auto'>
            {transformConversations(conversations).map((conversation) => ( 
              <Conversation key ={conversation.id}
              conversation={conversation}
              socket={socket}
              />

           ))} 
           {loading ? <Flex align="center" gap="middle">
                      <Spin size="small" />
                      <Spin />
                      <Spin size="large" />
                      </Flex> : null}


        </div>
    );
});
export default Conversations