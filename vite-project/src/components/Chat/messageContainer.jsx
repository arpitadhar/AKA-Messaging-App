import MessageInput from "./messageInput";
import Messages from "./Messages"
import useConversation from "../../hooks/useConversation"; 


const MessageContainer = ({socket}) => {
    const {selectedConversation, setSelectedConversation} = useConversation(); 
    

    /*
        This is where the app kept breaking. We attempted to update the message
        container once the server sent a "message-rec", but our hooks couldn't 
        be called within that function.
    */
    socket.on("message-rec", (msg) => {
        console.log("SOCKET BROADCAST RECEIVED:", msg);
        //Ideally the message container update would've happened here
    });

    return(
        <div className='md:min-w-[450px] flex flex-col' id = "mContainer">
            {!selectedConversation ? (
                <NoChatSelected/>
            ):(
                <>
                <div className='bg-slate-500 px-4 py-2 mb-2' id = "to">
                <span className='label-text'>To: </span>{""}
                <span className='text-gray-900 font-bold'>{selectedConversation.user2_id}</span>
                </div>
                <Messages/>
                <MessageInput socket={socket}/> {}
                </>
            )}
            
        </div>
    )
}

export default MessageContainer; 

const NoChatSelected = () => {
    return(
        <div className = 'flex items-center w-full h-full'>
            <div className='px-4 text-center sm:text-lg md:text-xl text-gray-200 font-semibold flex flex-col'>
                Welcome 
            </div>
        </div>
    )
}
