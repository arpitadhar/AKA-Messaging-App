import { IoSearch } from "react-icons/io5";

const SearchInput = () =>{
    return(
        <form className='flex gap-2' id = "search">
            <input type = 'text' placeholder ='..Search..' className = 'input input-bordered rounded-full' id ="searchInput"/>
            <button type='submit' className='btn btn-circle bg-transparent text-white'>
            <IoSearch className="searchBar"/>
            </button>

        </form>
    )
}
export default SearchInput