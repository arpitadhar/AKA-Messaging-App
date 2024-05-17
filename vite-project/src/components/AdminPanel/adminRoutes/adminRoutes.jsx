import { BrowserRouter, Routes} from "react-router-dom"; 
import AdminPanel from "../adminPanel";
import Users from "./users"; 
import Analytics from "./analytics"; 
import Settings from "./settings"; 

function AdminRoutes(){
    return(
        <BrowserRouter>
           <Routes>
            <Route path = "/" element={<AdminPanel/>}></Route>
            <Route path = "/users" element={<Users/>}></Route>
            <Route path = "/analytics" element={<Analytics/>}></Route>
            <Route path = "/settings" element={<Settings/>}></Route>
  
           </Routes>
        </BrowserRouter>
    )
}