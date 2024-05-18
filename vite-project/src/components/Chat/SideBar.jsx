import Conversations from "./conversations";
import SearchInput from "./searchInput"; 
import { useEffect, useState } from "react";
const Sidebar = () => {
    return(
        <div className='border-r' id = "sidebar"> 
            <SearchInput/>
            <div className = 'divider3'></div>
            <Conversations/>
        </div>
    )
}

export default Sidebar