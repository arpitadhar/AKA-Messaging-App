const Message = () => {
   return(
    <div>
    <div className='chat chat-end bg-gray-200 bg-opacity-50'>
        <div className='chat-image avatar'>
            <div className='w-10 rounded-full'>
            <span>JD</span>
            </div>
        </div>
        <div className={`chat-bubble text-white bg-blue-500`} id = 'messagesContainer'>Hi! Hello!</div>
        <div className='chat-footer opacity-50 text-xs flex gap-1 items-center'>12:42</div>
    </div>
    <div className="chat chat-end">
   <div className="chat-header2"></div>
   <div className="chat-bubble2" id = 'messagesContainer'>Hello!</div>
   <div className="chat-footer opacity-50" id = "footer2">
    Seen at 12:46
   </div>
</div>
    </div>
   )
}
export default Message; 