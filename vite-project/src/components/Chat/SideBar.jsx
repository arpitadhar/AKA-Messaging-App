import React, {useRef} from "react";
import Conversations from "./conversations";
import SearchInput from "./searchInput"; 
import { useEffect, useState } from "react";
const Sidebar = ({socket}) => {
    const conversationsRef = useRef();

    return(
        <div className='border-r' id = "sidebar"> 
            <SearchInput getConversations={() => conversationsRef.current.getConversations()} />
            <div className = 'divider3'></div>
            <Conversations ref={conversationsRef} socket={socket} />
        </div>
    )
}

export default Sidebar