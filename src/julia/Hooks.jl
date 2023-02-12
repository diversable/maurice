module Hooks

function new_script_posthook(script_name::String)
    try
        println("hello from the new script posthook! You gave me the script name: `$script_name`")
        # return "success"
        return 0
    catch
        # return "error"
        return 1
    end

end


end # module Hooks