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
     <div className = "container">
        <form action="">
        <div className = "header">
            <div className = "text">Welcome</div>
            <div className = "underline"></div>
        </div>
        <br></br>
        <div>
        <button><a href = "/login">Login</a></button>
        </div>
        <br></br>
        <div>
        <button><a href="/register">Register</a></button>
        </div>
        <br></br>
        <div>
        <button><a href = "/aboutus">About Us</a></button>
        </div>
        </form>
      </div>
   }
   </div>
   )
}