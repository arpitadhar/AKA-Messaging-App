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
         <h1 class="display-1 text-white">AKA Messaging</h1>
         <div id="outer-display">
             <div id="inner-display">
             </div>
         </div>
         <div id="outer-input">
             <div>
                 <form>
                     <h2 class="text-white"><label for="display-name">Display Name:</label></h2>
                     <h5 class="text-white">@newUser</h5>
                 </form>
                 <h2 class="text-white">Message:</h2>
                 <textarea type="text" id="message"></textarea>
                 <br/>
                 <button id="submitPost" class="text-white btn btn-success"> Post </button>
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