import { useState, useEffect } from "react";
import { Link, useNavigate } from "react-router-dom";
import "./LandingPage.css"

export default function LandingPage(props) {
   //console.log(props.user);
   const navigate = useNavigate(); 
   
   return (
      <div>
      {
         props.user ? 

         <div id="root">
         <div class="container">
        <div class="sidebar">
            <h1>AKA</h1>
            <h2>Start new conversation</h2>
            <form id="createConversationForm">
                <input type="text" id="receiverName" name="receiverName" placeholder="Search for user..."/>
                <button type="submit">Start Conversation</button>
            </form>
            <div id="conversationListContainer">
                <h2>Your Conversations</h2>
                <ul class = "conversation-list" id = "conversationsList" >
                </ul>
            </div>
        </div>
        <div class="main">
            <div id="inner-display"></div>
            <div class="form">
                <form id="sendMessageForm">
                    <textarea name="message" id="message" placeholder="Type your message here"></textarea>
                    <button type="submit">Send</button>
                </form>
            </div>
        </div>
    </div>
    </div>
     :
     <div className="landing-page">
            <div className="landing-page-content" >
               <h1 class="display-1 text-white">Incredibly high quality and relevent Hero to attract new users goes here</h1>
               <h1 class="display-3 text-white">Maybe even an 'about us' section if we're feeling fancy, we could make it scrollable and everything</h1>
               <h1 class="display-3 text-white">It'll let us populate the navbar at the very least</h1>
            </div>
         </div>
   }
   </div>
   )
}