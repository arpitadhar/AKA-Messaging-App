import MessageInput from "./messageInput";
import Messages from "./Messages"
import useConversation from "../../hooks/useConversation"; 
import { useNavigate } from 'react-router-dom'; 
import { TinyColor } from '@ctrl/tinycolor';
import { Button, ConfigProvider, Space } from 'antd';


const MessageContainer = () => {
    const {selectedConversation, setSelectedConversation} = useConversation(); 
    const navigate = useNavigate();  

    // let whichUser = ""; 
    // if(selectedConversation.user2_id === user){
    //   whichUser = selectedConversation.user1_id; 
    // }
    // else{
    //   whichUser = selectedConversation.user2_id; 
    // }
    return(
        <div className='md:min-w-[450px] flex flex-col' id = "mContainer">
            {!selectedConversation ? (
                <NoChatSelected/>
            ):(
                <>
                <div className='bg-slate-500 px-4 py-2 mb-2' id = "to">
                <span className='label-text'>To: </span>{""}
                <span className='text-gray-900 font-bold'>{selectedConversation.user2_id}</span>
                <button onClick={(e) =>{ e.preventDefault(); navigate('user');}}>Your Profile</button>
                </div>
                <Messages/>
                <MessageInput />
                </>
            )}
            
        </div>
    )
}

export default MessageContainer; 

const NoChatSelected = () => {
    const navigate = useNavigate();  
    return(
        <div className = 'flex items-center w-full h-full'>
            <div className='px-4 text-center sm:text-lg md:text-xl text-gray-200 font-semibold flex flex-col'>
                Welcome 
            </div>
            <button onClick={(e) =>{ e.preventDefault(); navigate('user');}}>Your Profile</button>
        </div>
    )
}