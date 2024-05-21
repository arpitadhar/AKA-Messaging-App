import {useState, useEffect} from 'react'; 
import "./conversation.css"; 
import useConversation from "../../hooks/useConversation"; 

const Conversation = ({conversation, socket}) => {
    const {selectedConversation, setSelectedConversation} = useConversation(); 
    const isSelected = selectedConversation?.id === conversation.id; 
    const user1 = sessionStorage.getItem("email"); 
    const user = user1.replace(/^"(.*)"$/, '$1');
    const handleSelectedConversation = () => {
      setSelectedConversation(conversation); 
      socket.emit("join",conversation.id);
    };  
    console.log(selectedConversation);
    let whichUser = ""; 
    if(conversation.user2_id === user){
      whichUser = conversation.user1_id; 
    }
    else{
      whichUser = conversation.user2_id; 
    }
    return<>
        <div className={`flex gap-2 items-center hover:bg-sky-500 rounded p-2 py-1 cursor-pointer ${isSelected ? "bg-blue" : ""}`} id = "conversation" onClick = {handleSelectedConversation}>
           <div className="avatar placeholder" id = "avatar">
              <div  className="bg-neutral text-neutral-content rounded-full w-16" id="convo-name">
                 <span className="text-xl">JD</span>
               </div>
            </div>

           <div className='flex flex-col flex-1'>
            <div className='flex gap-3 justify-between' id="fullConvo">
                <p className='font-bold text-white'>{whichUser}</p>
                <span className='text-x1'></span>
            </div>
          </div>
        </div>
        <div className='divider my-0 py-0 h-1' id="convo-divider" />
    
        </>;
    
}; 
export default Conversation