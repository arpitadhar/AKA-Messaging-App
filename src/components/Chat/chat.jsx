import "./chat.css"; 
import MessageContainer from "./messageContainer";
import SearchInput from "./searchInput"; 
import Sidebar from "./SideBar"; 
export default function Chat(props) {
    //console.log(props.user);
    //const navigate = useNavigate(); 
    
    return (
      <div className = "chat_body">
      <div className='flex sm:h-[450px] md:h-[550px] rounded-lg overflow-hidden bg-gray-400 bg-clip-padding backdrop-filter backdrop-blur-lg bg-opacity-0'>
        <Sidebar/>
      </div>
      <MessageContainer/>
      </div>
//         <div class="container" id = "chat-page">
//         <div class = "sidebar">
//           Conversations: 
//           <form id="createConversationForm">
//            <input type="text" id="recieverName" name="recieverName" placeholder="Search for user.."></input>
//            <button type="submit">Start Conversation</button>
//           </form>
//           <div id="conversationListContainer">
//             <h2>Your Conversations</h2>
//             <ul></ul>
//           </div>
//         </div>
//         <div class="main">
//         <div id="inner-display"></div>
//         <div className="chat chat-start">
//   <div className="chat-image avatar">
//     <div className="w-10 rounded-full">
//       <img alt="Tailwind CSS chat bubble component" src="https://daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" />
//     </div>
//   </div>
//   <div className="chat-header">
//     Obi-Wan Kenobi
//     <time className="text-xs opacity-50">12:45</time>
//   </div>
//   <div className="chat-bubble">You were the Chosen One!</div>
//   <div className="chat-footer opacity-50">
//     Delivered
//   </div>
// </div>
// <div className="chat chat-end">
//   <div className="chat-image avatar">
//     <div className="w-10 rounded-full">
//       <img alt="Tailwind CSS chat bubble component" src="https://daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" />
//     </div>
//   </div>
//   <div className="chat-header">
//     Anakin
//     <time className="text-xs opacity-50">12:46</time>
//   </div>
//   <div className="chat-bubble">I hate you!</div>
//   <div className="chat-footer opacity-50">
//     Seen at 12:46
//   </div>
// </div>
//                <div class="form">
//                    <form id="sendMessageForm">
//                        <textarea name="message" id="message" placeholder="Type your message here"></textarea>
//                        <button type="submit">Send</button>
//                    </form>
//                </div>
//         </div>
//        </div>
    )
 }