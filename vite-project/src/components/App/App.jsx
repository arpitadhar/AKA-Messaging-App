import { useEffect, useState } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import "bootstrap/dist/css/bootstrap.min.css";
import LoginPage from "../LoginPage/LoginPage";
import RegistrationPage from "../RegistrationPage/RegistrationPage";
import NotFound from "../NotFound/NotFound";
import AccessForbidden from "../AccessForbidden/AccessForbidden";
import LandingPage from "../LandingPage/LandingPage";
import AboutUs from "../AboutUs/aboutus"
import './App.css';
import Navbar from "../Navbar/Navbar";
import AdminPanel from "../AdminPanel/adminPanel";
import Chat from "../Chat/chat";
import Users from "../AdminPanel/adminRoutes/users"; 
import ForgotPassword from "../ForgotPassword/forgot";
import {} from "antd"; 
import VerifyCode from "../ForgotPassword/verify_code";

export default function AppContainer() {
  return (
      <App />
  );
}


function App() {
  const [user, setUser] = useState(false); 
  const [isLoggedIn, setIsLoggedIn] = useState(false); 
  const handleLogout = () => {
    sessionStorage.clear(); 
    setIsLoggedIn(false); 
  };
  return (
    <div className="App">
      <BrowserRouter>
        <Navbar user={user}
                setUser={setUser}
                isLoggedIn = {isLoggedIn} handleLogout ={handleLogout}/>
        <Routes>
          <Route path="/" element={<LandingPage                                             
                                            user={user}
                                            setUser={setUser} />} />
          <Route path="/login" element={<LoginPage                                             
                                            user={user}
                                            setUser={setUser} setIsLoggedIn ={setIsLoggedIn} isLoggedIn = {isLoggedIn} />} />
          <Route path="/register" element={<RegistrationPage 
                                            user={user}
                                            setUser={setUser} />} />
          <Route path="/forgot" element={<ForgotPassword />} />
          <Route path="/verify" element={<VerifyCode />} /> 
          <Route path="/chat" element={<Chat isLoggedIn = {isLoggedIn}/>} />
          <Route path="/aboutus" element={<AboutUs/>} />
          <Route path="/adminpanel" element={<AdminPanel/>} />
          <Route path="/users" element={<Users/>} />
          <Route path="*" element={<NotFound />} />
          <Route path="/denied" element={<AccessForbidden />} />
        </Routes>
      </BrowserRouter>
    </div>
  );
}

