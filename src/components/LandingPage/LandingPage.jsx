import { useState, useEffect } from "react";
import { Link, useNavigate } from "react-router-dom";
import "./LandingPage.css"

export default function LandingPage(props) {
   //console.log(props.user);
   const navigate = useNavigate(); 
   
   return (
      <div style={{ fontFamily: 'Rubik, sans-serif' }}>
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
     <div className = "containerLanding" style={{ fontFamily: 'Rubik, sans-serif' }}>
        <div className="welcome-div" >
        <h1 className="welcome">Welcome</h1>
        </div>
        <div className="under_welcome">
        <Link to="/login"><button className="btn glass">Login</button></Link>
        <br></br>
        <div>
        <Link to="/register"><button className="btn glass">Register</button></Link>
        </div>
        <Link to="/aboutus"><button className="btn glass">About Us</button></Link>
        </div>
      </div>
   }
   </div>
   )
}