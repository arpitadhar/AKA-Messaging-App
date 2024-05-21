import { useEffect, useState } from "react";

const FirstName = () =>{
    const [first, setFirst] = useState(""); 
    let data2 = localStorage.getItem("user-info");
    setFirst(data2.first_name); 
    return(
        <div className="col-md-6">
        <label className="form-label">First Name*</label>
        <input type="text" className="form-control" placeholder={first} aria-label="First name" />
        <button>Change First Name</button>
    </div>
    )
}

export default FirstName;
