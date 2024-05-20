import * as React from "react";
import { useEffect, useState } from "react";
import { Link, Navigate, useNavigate } from "react-router-dom";
import AccessForbidden from "../AccessForbidden/AccessForbidden";
import { useRef } from 'react';
import "./UserProfile.css";
import { contentQuotesLinter } from "@ant-design/cssinjs/lib/linters";


export default function UserProfile({isLoggedIn})  {
    //Hacky way of implementation, lift this props to App component
    const [firstName, setFirstName] = useState("");
    const [lastName, setLastName] = useState("");
    const [oldPassword, setOldPassword] = useState("")
    const [password, setPassword] = useState("");
    const [confirmPassword, setConfirmPassword] = useState("");
    const [first, setFirst] = useState(""); 
    const [last, setLast] = useState(""); 
    const [url, setUrl] = useState("");
    const [username, setUsername] = useState(""); 
    const [email1, setEmail] = useState(""); 
    const user1 = localStorage.getItem("email"); 
    const user = user1.replace(/^"(.*)"$/, '$1');
    const infoRequest = fetch("http://localhost:3000/user-info", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify( user )
    })
    .then(response => {
        if (!response.ok) {
            throw new Error("Network response was not ok");
        }
        return response.json();
    })
    .then(data => {
      console.log(data);
      localStorage.setItem("user-info", JSON.stringify(data)); // Assuming email is returned from the server
      console.log(localStorage.getItem("user-info"));
      console.log("???"); 
      let data2 = localStorage.getItem("user-info");
      //console.log(localStorage.getItem("user-info").first_name);
      let info = JSON.parse(data2); 
      console.log(data.first_name);
      setFirst(data.first_name); 
      setLast(data.last_name); 
      setUsername(data.username); 
      setEmail(data.email);
      let userInfo = localStorage.getItem("user-info");
      console.log(userInfo.first_name); 
     })
    .catch(error => {
        console.error("Error retrieving info:", error);
        // Display error message as an alert
        alert("Error retrieving info: " + error.message);
    });
    

      //Updates image when changes happen
  // const imageChange = (e) => {
  //   if (e.target.files && e.target.files.length > 0) {
  //     setSelectedImage(e.target.files[0]);
  //     setImageError({});
  //   }
  // };

  // //Removes image preview if clicked on
  // const removeSelectedImage = () => {
  //   setSelectedImage();
  // };

  // // open file explorer/finder to select image
  // const inputRef = useRef(null);

  // const handleClick = () => {
  //   inputRef.current.click();
  // };

  // //Updates changes for file values
  // const handleFileChange = (e) => {
  //   const fileObj = e.target.files && e.target.files[0];
  //   if (!fileObj) {
  //     return;
  //   }
  // };
  // //Submit function that is called when user submits form
  // const handleOnSubmit = () => {
  //   //Validation before sending to backend
  //   if (selectedImage === undefined) {
  //     setImageError({ image: "Select an Image" });
  //     return;
  //   }
  // }

    //Sends request to make a post 
    // const uploadImage = async (data) => {
    //     setIsLoading(true);
    //     try {
    //       const getData = await ApiClient.uploadImage(data);

    //     } catch (err) {
    //       setError(err);
    //       console.error(err);
    //     }
    //     setIsLoading(false);
    //   };

    // //Sends request to make a post 
    // const getImage = async (data) => {
    //     setIsLoading(true);
    //     try {
    //       const getData = await ApiClient.getImage(data);

    //     } catch (err) {
    //       setError(err);
    //       console.error(err);
    //     }
    //     setIsLoading(false);
    //   };
    //TODO
    //I really don't wanna add another input box, didn't realize that there was no username change going on until it was too late
    //TODO

    //const handleUsernameChange = (event) => {
    //    setUsername(event.target.value);
    //}
    
    const handleFirstNameChange = (event) => {
        setFirstName(event.target.value);
        const input_name = document.getElementById("change_firstName").value;
        const infoRequest = fetch("http://localhost:3000/update-first", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({email: email1, input: input_name})
        })
        .then(response => {
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            return response.json();
        })
        .then(data => {
          console.log(data);
          localStorage.setItem("user-info", JSON.stringify(data)); 
          console.log(localStorage.getItem("user-info"));
          console.log("???"); 
          setFirst(data.first_name); 
          //console.log(userInfo.first_name); 
         })
        .catch(error => {
            console.error("Error changing first:", error);
            alert("Error changing first: " + error.message);
        });

    }

    const handleLastNameChange = (event) => {
        const input_name = document.getElementById("change_lastName").value;
        const infoRequest = fetch("http://localhost:3000/update-last", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({email: email1, input: input_name})
        })
        .then(response => {
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            return response.json();
        })
        .then(data => {
          console.log(data);
          localStorage.setItem("user-info", JSON.stringify(data)); 
          console.log(localStorage.getItem("user-info"));
          console.log("???"); 
          setLast(data.last_name); 
          //console.log(userInfo.first_name); 
         })
        .catch(error => {
            console.error("Error changing last:", error);
            alert("Error changing last: " + error.message);
        });
    }
    //NEW IMAGE
    const handleImgChange = (event) => {
        const input_url = document.getElementById("change_imgUrl").value;
        const infoRequest = fetch("http://localhost:3000/update-img", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({email: email1, input: input_url})
        })
        .then(response => {
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            return response.json();
        })
        .then(data => {
          console.log(data);
          localStorage.setItem("user-info", JSON.stringify(data)); 
          console.log(localStorage.getItem("user-info"));
          console.log("???"); 
          setUrl(data.img_url);
         })
        .catch(error => {
            console.error("Error changing first:", error);
            alert("Error changing first: " + error.message);
        });

    }

    const handleUsernameChange = (event) => {
        const input_name = document.getElementById("newUsername").value;
        const confirm_name = document.getElementById("confirmUsername").value;
        if (confirm_name == input_name){
            const infoRequest = fetch("http://localhost:3000/update-username", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({email: email1, input: input_name})
          })
            .then(response => {
                if (!response.ok) {
                throw new Error("Network response was not ok");
                }
                return response.json();
            })
            .then(data => {
          console.log(data);
          localStorage.setItem("user-info", JSON.stringify(data)); 
          console.log(localStorage.getItem("user-info"));
          console.log("???"); 
          setUsername(data.username); 
          //console.log(userInfo.first_name); 
         })
        .catch(error => {
            console.error("Error changing username:", error);
            alert("Error changing username: " + error.message);
        });
        }
        else{
            alert("usernames don't match"); 
        }
        
    }


    const handlePasswordChange = (event) => {
        
    }


    //const handleSubmit = (event) => {
    //    event.preventDefault();
//
    //    document.getElementById('invalid-0').classList.add("d-none");
    //    document.getElementById('invalid-1').classList.add("d-none");
    //
    //    const oldPasswordBox = document.getElementById('oldPassword');
    //    const confirmPasswordBox = document.getElementById('confirmPassword');
//
    //    if(password) {
    //        if(oldPassword){
    //            //send to backend to check if correct
    //            //if true continue, if false throw error message
    //            return;
    //        } else {
    //            oldPasswordBox.classList.add('border-danger');
    //            document.getElementById('invalid-0').classList.remove("d-none");
    //        }
    //        if(password == confirmPassword) {
    //            //send to backend to replace current password
    //            return;
    //        } else {
    //            confirmPasswordBox.classList.add('border-danger');
    //            document.getElementById('invalid-1').classList.remove("d-none");
    //        }
//
    //    }
        
    //}

    // const UserProfileImage = getImage()
    
    const [loggedInUser, setLoggedInUser] = useState(null);
    const navigate = useNavigate();

    useEffect(() => {
        const user = localStorage.getItem("user");
        if (user) {
            setLoggedInUser(JSON.parse(user));
        } 
    }, []);

    
    if (isLoggedIn) {
        return (
            <body id = "user-profile">
            <div className="container text-white" style={{ fontFamily: 'Rubik, sans-serif' }}>
            <div className="row">
            		<div className="col-12">
            			<div className="my-5">
            				{/* <h3>{loggedInUser.email}'s Profile</h3> */}
                            <h3>{first}'s Profile</h3>
                            {/* <ProfileImage
                            source={toBase64(UserProfileImage?.photo?.data)}
                            />  */}
            				<hr/>
            			</div>
            			<form className="file-upload">
            				<div className="row mb-5 gx-5">
            					<div className="col-xxl-8 mb-5 mb-xxl-0">
            						<div className="bg-secondary-soft px-4 py-5 rounded">
            							<div className="row g-3">
            								<h4 className="mb-4 mt-0">Personal Info</h4>
            								<div className="col-md-6">
            									<label className="form-label">First Name*</label>
            									<input type="text" id="change_firstName"className="form-control" placeholder={first} aria-label="First name"/>
                                                <br></br>
                                                <button id = "change_first" className = "userButtons" onClick={handleFirstNameChange}>Change First Name</button>
            								</div>
            								<div className="col-md-6">
            									<label className="form-label">Last Name *</label>
            									<input type="text"id="change_lastName" className="form-control" placeholder={last} aria-label="Last name"/>
                                                <br></br>
                                                <button id = "change_last" onClick={handleLastNameChange}>Change Last Name</button>
            								</div>
            							</div> 
            						</div>
            					</div>
            					<div className="col-xxl-4">
            						<div className="bg-secondary-soft px-4 py-5 rounded">
            							<div className="row g-3">
                                            <img className="profileImg"src={url ? url:""} />
            								<h4 className="mb-4 mt-0">Upload your profile photo</h4>
            								<div className="text-center">
            									<div className="square position-relative display-2 mb-3">
            										<i className="fas fa-fw fa-user position-absolute top-50 start-50 translate-middle text-secondary"></i>
            									</div>
            									<input type="test" id="change_imgUrl" className="form-control" hidden=""/>
            									<label className="btn btn-success btn-block" onClick={handleImgChange}>Upload</label>
                                                <br></br>
            									<button type="button" id = "remove" className="btn btn-danger btn-sm">Remove</button>
            									<p className="text-muted mt-3 mb-0"><span class="me-1">Note:</span>Minimum size 300px x 300px</p>
            								</div>
            							</div>
            						</div>
            					</div>
            				</div> 
                            <div className="row mb-5 gx-5">
            					<div className="bg-secondary-soft px-4 py-5 rounded">
            						<div className="row g-3">
            							<h4 className="my-4">Change Username</h4>
            							<div class="col-md-12">
            								<label for="confirmPassword" className="form-label" >Enter New Username Here *</label>
            								<input type="text" className="form-control" id="newUsername" placeholder = {username}/>
            							</div>
                                        <div class="col-md-12">
            								<label for="confirmPassword" className="form-label">Confirm Username *</label>
            								<input type="text" className="form-control" id="confirmUsername"/>
                                            <br></br>
                                            <button type="button" id = "new_username" className="btn btn-danger btn-sm" onClick = {handleUsernameChange}>Confirm Username</button>
            							</div>
            						</div>
            					</div>
            				</div>
            				<div className="row mb-5 gx-5">
            					<div className="bg-secondary-soft px-4 py-5 rounded">
            						<div className="row g-3">
            							<h4 className="my-4">Change Password</h4>
            							<div className="col-md-6">
            								<label for="oldPassword" className="form-label">Old password *</label>
            								<input type="password" className="form-control" id="oldPassword"/>
                                            <div id="invalid-0" class="text-danger d-none">
                                            Please enter your password
                                            </div>
            							</div>
            							<div className="col-md-6">
            								<label for="newPassword" className="form-label">New password *</label>
            								<input type="password" className="form-control" id="newPassword" />
            							</div>
            							<div class="col-md-12">
            								<label for="confirmPassword" className="form-label">Confirm Password *</label>
            								<input type="password" className="form-control" id="confirmPassword"/>
                                            <div id="invalid-1" class="text-danger d-none">
                                            Passwords do not match
                                            </div>
                                            <br></br>
                                            <button type="button" className="btn btn-danger btn-lg">New Password</button>
                                            <br></br>
                                            <br></br>
                                            <br></br>
                                            <br></br>
                                            <button type="button" id = "remove-profile" className="btn btn-danger btn-lg">Delete profile</button>
            							</div>
            						</div>
            					</div>
            				</div>
            			</form> 
            		</div>
            	</div>
            </div>
            </body>
        );
     }
        else {
            return(
            <AccessForbidden />
             );
    }
}