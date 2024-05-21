import { useEffect, useState } from "react";
import "./chat.css"; 
import MessageContainer from "./messageContainer";
import Sidebar from "./SideBar"; 
import useConversation from "../../hooks/useConversation";

export default function Chat({ isLoggedIn,socket:propSocket }) {
    const socket = useConversation(state=>state.socket);
    useEffect(() => {
      console.log("isLoggedIn in Chat component:", isLoggedIn);
    }, [isLoggedIn]);
    if (isLoggedIn) {
        return (
            <div className="chat_body">
                <div className='flex sm:h-[450px] md:h-[550px] rounded-lg overflow-hidden bg-gray-400 bg-clip-padding backdrop-filter backdrop-blur-lg bg-opacity-0'>
                    <Sidebar socket={propSocket}/>
                </div>
                <MessageContainer socket={propSocket}/>
            </div>
        );
    } else {
        return (
            <div className="access-forbidden">
                <div className="content">
                    <h1>You can't view this page.</h1>
                    <p>Log In or Register to view this page.</p>
                </div>
            </div>
        );
    }
}