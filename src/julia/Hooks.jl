module Hooks
# All functions must return either a "0" ('success') or "1" (error) value in order to work with the Maurice (mce) app...

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