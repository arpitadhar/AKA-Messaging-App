import "./conversation.css"

const Conversation = () => {
    return<>
        <div className='flex gap-2 items-center hover:bg-sky-500 rounded p-2 py-1 cursor-pointer' id = "conversation">
           <div className="avatar placeholder" id = "avatar">
              <div  className="bg-neutral text-neutral-content rounded-full w-16" id="convo-name">
                 <span className="text-xl">JD</span>
               </div>
            </div>

           <div className='flex flex-col flex-1'>
            <div className='flex gap-3 justify-between' id="fullConvo">
                <p className='font-bold text-white'>John Doe</p>
                <span className='text-x1'></span>
            </div>
          </div>
        </div>
        <div className='divider my-0 py-0 h-1' id="convo-divider" />
    
        </>;
    
}; 
export default Conversation