import { useEffect } from "react";
import "./chat.css"; 
import MessageContainer from "./messageContainer";
import SearchInput from "./searchInput"; 
import Sidebar from "./SideBar"; 
export default function Chat(props) {
    //console.log(props.user);
    //const navigate = useNavigate(); 
    function lightMode() {
        var element = document.chat_body; 
        element.classList.toggle("dark-mode"); 
    }
    return (
      <div className = "chat_body">
      <div className='flex sm:h-[450px] md:h-[550px] rounded-lg overflow-hidden bg-gray-400 bg-clip-padding backdrop-filter backdrop-blur-lg bg-opacity-0'>
        <Sidebar/>
      </div>
      <MessageContainer/>
      </div>
    )
 }