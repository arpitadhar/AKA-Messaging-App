import { useEffect, useState } from "react";
import "./chat.css"; 
import MessageContainer from "./messageContainer";
import SearchInput from "./searchInput"; 
import Sidebar from "./SideBar"; 
import { useNavigate } from 'react-router-dom'; 
import { TinyColor } from '@ctrl/tinycolor';
import { Button, ConfigProvider, Space } from 'antd';

// const navigate = useNavigate();
export default function Chat({ isLoggedIn }) {
    //console.log(props.user);
    //const navigate = useNavigate(); 
    useEffect(() => {
      console.log("isLoggedIn in Chat component:", isLoggedIn);
    }, [isLoggedIn]);
    if (isLoggedIn) {
        return (
            <div className="chat_body">
                <div className='flex sm:h-[450px] md:h-[550px] rounded-lg overflow-hidden bg-gray-400 bg-clip-padding backdrop-filter backdrop-blur-lg bg-opacity-0'>
                    <Sidebar/>
                </div>
                <MessageContainer/>
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