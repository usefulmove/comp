#!julia

const COMP_VERSION = "0.13.10"

#=

    note: base data structure is a vector (linked
    list) used as a stack. atoms on the list are
    either be symbols (commands) or values. each
    calculation is a list of operations that are
    processed in order of occurrence. this is an
    implementation of a lisp interpreter for rev-
    erse polish notation s-expressions (sexp).

      operations list structure
        (object : command or value)
        "5"
        "sqrt"
        "1
        "-"
        "2"
        "/"

    a list evaluation engine takes the list of
    strings and executes the corresponding oper-
    ations then returns the resulting mutated
    stack.

=#

function julia_main()::Cint
  try
    arg = ARGS # argument vector
    #println(string(arg)) # debug

    length(arg) == 0 ? push!(arg, "help") : nothing

    if arg[1] == "--help" || arg[1] == "help"
      println("usage: comp [version] [help]")
      println("       comp <list>")
      println("       comp -f <file>")
      println()
      println("where <list> represents a sequence of reverse Polish notion (rpn) \
      postfix operations or <file> is a file containing a similar sequence of \
      operations. Each operation must be either a command (symbol) or value. As \
      examples, 'comp 3 4 +' adds the values 3 and 4 and '3 dup x 4 dup x +' \
      computes the sum of the squares of 3 and 4. The available commands are \
      listed below.")
      println()
      println("commands:")
      for c in sort(collect(keys(commands)))
        print(c, " ")
      end
      println()
    elseif arg[1] == "--version" || arg[1] == "version"
      println("comp ", COMP_VERSION)
    elseif arg[1] == "-f"
      # build vector with file contents
      ops = Vector{String}()
      o = read(arg[2], String)
      o = split(o, (' ','\n'))
      for i = 1:length(o)
        if length(o[i]) > 0
          push!(ops, o[i])
        end
      end
      main(ops)
    elseif arg[1] == "mona"
      println(mona)
    else
      main(arg)
    end
  catch
    Base.invokelatest(Base.display_error, Base.catch_stack())
    return 1
  end

  return 0
end

function main(oplist::Vector{String})
  # create computation stack
  cstack = Vector{Float64}(undef, 0)
  # evaluate list of arguments and update stack by
  # mapping node evaluation to the operations list
  map(x -> processnode!(cstack, x), oplist)
  # return resulting stack after argument list evaluation
  for e in cstack
    println(string(e))
  end
end

# execute command function or add value to stack
function processnode!(stack::Vector{Float64}, cmdval::String)
  cmdval in keys(commands) ?
  eval(commands[cmdval])(stack) :
  c_addtostack!(stack, cmdval)
  return nothing
end

function c_addtostack!(s::Vector{Float64}, cov::String)
  push!(s, parse(Float64, cov))
end

# -- create command dictionary and define commands and behaviors ---------------

commands = Dict{String, Symbol}()

# -- stack manipulation --------------------------------------------------------

# - drop
commands["drop"] = :c_drop!
function c_drop!(s::Vector{Float64})
  pop!(s)
end

# - duplicate
commands["dup"] = :c_dup!
function c_dup!(s::Vector{Float64})
  push!(s, s[end])
end

# - swap x and y
commands["swap"] = :c_swap!
function c_swap!(s::Vector{Float64})
  x = s[end]
  s[end] = s[end-1]
  s[end-1] = x
end

# - clear stack
commands["cls"] = :c_cls!
commands["clr"] = :c_cls!
function c_cls!(s::Vector{Float64})
  empty!(s)
end

# - roll stack
commands["roll"] = :c_roll!
function c_roll!(s::Vector{Float64})
  o = pop!(s)
  pushfirst!(s, o)
end

# -- memory usage --------------------------------------------------------------

# - save/retrieve a
global stor_a = 0.0

commands[".a"] = :c_store_a!
commands["sa"] = :c_store_a!
function c_store_a!(s::Vector{Float64})
  global stor_a = pop!(s)
end

commands["a"] = :c_push_a!
function c_push_a!(s::Vector{Float64})
  push!(s, stor_a)
end

# - save/retrieve b
global stor_b = 0.0

commands[".b"] = :c_store_b!
commands["sb"] = :c_store_b!
function c_store_b!(s::Vector{Float64})
  global stor_b = pop!(s)
end

commands["b"] = :c_push_b!
function c_push_b!(s::Vector{Float64})
  push!(s, stor_b)
end

# - save/retrieve c
global stor_c = 0.0

commands[".c"] = :c_store_c!
commands["sc"] = :c_store_c!
function c_store_c!(s::Vector{Float64})
  global stor_c = pop!(s)
end

commands["c"] = :c_push_c!
function c_push_c!(s::Vector{Float64})
  push!(s, stor_c)
end

# -- math operations -----------------------------------------------------------

# - add
commands["+"] = :c_add!
function c_add!(s::Vector{Float64})
  s[end-1] += pop!(s)
end

# - add all
commands["+_"] = :c_add_all!
function c_add_all!(s::Vector{Float64})
  while length(s) > 1
    s[end-1] += pop!(s)
  end
end

# - subtract
commands["-"] = :c_sub!
function c_sub!(s::Vector{Float64})
  s[end-1] -= pop!(s)
end

# - multiply
commands["x"] = :c_mult!
function c_mult!(s::Vector{Float64})
  s[end-1] *= pop!(s)
end

# - multiply all
commands["x_"] = :c_mult_all!
function c_mult_all!(s::Vector{Float64})
  while length(s) > 1
    s[end-1] *= pop!(s)
  end
end

# - divide
commands["/"] = :c_div!
function c_div!(s::Vector{Float64})
  s[end-1] /= pop!(s)
end

# - change sign
commands["chs"] = :c_chs!
function c_chs!(s::Vector{Float64})
  s[end] = -1 * s[end]
end

# - absolute value
commands["abs"] = :c_abs!
function c_abs!(s::Vector{Float64})
  s[end] = abs(s[end])
end

# - round (convert to integer)
commands["round"] = :c_round!
commands["int"] = :c_round!
function c_round!(s::Vector{Float64})
  s[end] = round(s[end])
end

# - invert
commands["inv"] = :c_inv!
function c_inv!(s::Vector{Float64})
  s[end] = 1 / s[end]
end

# - square root
commands["sqrt"] = :c_sqrt!
function c_sqrt!(s::Vector{Float64})
  s[end] = sqrt(s[end])
end

# - nth root
commands["throot"] = :c_throot!
function c_throot!(s::Vector{Float64})
  s[end-1] ^= 1 / pop!(s)
end

# - principal roots
commands["proot"] = :c_proot!
function c_proot!(s::Vector{Float64})
  c = pop!(s)
  b = pop!(s)
  a = pop!(s)

  if (b^2 - 4a*c) < 0 # complex solution
    push!(s, -b/2a) # root 1 real
    push!(s, sqrt(4a*c - b^2)/2a) # root 1 imag
    push!(s, -b/2a) # root 2 real
    push!(s, -1*sqrt(4a*c - b^2)/2a) # root 2 imag
  else
    push!(s, (-b+sqrt(b^2-4a*c))/2a) # root 1 real
    push!(s, 0) # root 1 imag
    push!(s, (-b-sqrt(b^2-4a*c))/2a) # root 2 real
    push!(s, 0) # root 2 imag
  end
end

# - exponentiation
commands["exp"] = :c_exp!
commands["^"] = :c_exp!
function c_exp!(s::Vector{Float64})
  s[end-1] ^= pop!(s)
end

# - modulus
commands["%"] = :c_mod!
commands["mod"] = :c_mod!
function c_mod!(s::Vector{Float64})
  divisor = pop!(s)
  s[end] = s[end] % divisor
end

# - factorial
commands["!"] = :c_factorial!
function c_factorial!(s::Vector{Float64})
  s[end] = factorial( Int64(s[end]) )
end

# - greatest common denominator
commands["gcd"] = :c_gcd!
function c_gcd!(s::Vector{Float64})
  a = pop!(s)
  b = pop!(s)

  push!(s, gcd(Int64(a), Int64(b)))
end

# - π
commands["pi"] = :c_pi!
function c_pi!(s::Vector{Float64})
  push!(s, π)
end

# - Euler's number (ℯ)
commands["e"] = :c_euler!
function c_euler!(s::Vector{Float64})
  push!(s, ℯ)
end

# - degrees to radians
commands["dtor"] = :c_dtor!
function c_dtor!(s::Vector{Float64})
  s[end] = s[end] * π / 180
end

# - radians to degrees
commands["rtod"] = :c_rtod!
function c_rtod!(s::Vector{Float64})
  s[end] = s[end] * 180 / π
end

# - sine
commands["sin"] = :c_sin!
function c_sin!(s::Vector{Float64})
  s[end] = sin(s[end])
end

# - arcsine
commands["asin"] = :c_asin!
function c_asin!(s::Vector{Float64})
  s[end] = asin(s[end])
end

# - cosine
commands["cos"] = :c_cos!
function c_cos!(s::Vector{Float64})
  s[end] = cos(s[end])
end

# - arccosine
commands["acos"] = :c_acos!
function c_acos!(s::Vector{Float64})
  s[end] = acos(s[end])
end

# - tangent
commands["tan"] = :c_tan!
function c_tan!(s::Vector{Float64})
  s[end] = tan(s[end])
end

# - arctangent
commands["atan"] = :c_atan!
function c_atan!(s::Vector{Float64})
  s[end] = atan(s[end])
end

# - log 10
commands["log"] = :c_log10!
commands["log10"] = :c_log10!
function c_log10!(s::Vector{Float64})
  s[end] = log10(s[end])
end

# - natural log
commands["ln"] = :c_ln!
function c_ln!(s::Vector{Float64})
  s[end] = log(s[end])
end

# -- mona ----------------------------------------------------------------------

mona = """
       !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!>''''''<!!!!!!!!!!!!!!!!!!!!!!!!!!!!
       !!!!!!!!!!!!!!!!!!!!!!!!!!!!'''''`             ``'!!!!!!!!!!!!!!!!!!!!!!!!
       !!!!!!!!!!!!!!!!!!!!!!!!''`          .....         `'!!!!!!!!!!!!!!!!!!!!!
       !!!!!!!!!!!!!!!!!!!!!'`      .      :::::'            `'!!!!!!!!!!!!!!!!!!
       !!!!!!!!!!!!!!!!!!!'     .   '     .::::'                `!!!!!!!!!!!!!!!!
       !!!!!!!!!!!!!!!!!'      :          `````                   `!!!!!!!!!!!!!!
       !!!!!!!!!!!!!!!!        .,cchcccccc,,.                       `!!!!!!!!!!!!
       !!!!!!!!!!!!!!!     .-\"?\$\$\$\$\$\$\$\$\$\$\$\$\$\$c,                      `!!!!!!!!!!!
       !!!!!!!!!!!!!!    ,ccc\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$,                     `!!!!!!!!!!
       !!!!!!!!!!!!!    z\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$;.                    `!!!!!!!!!
       !!!!!!!!!!!!    <\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$:.                    `!!!!!!!!
       !!!!!!!!!!!     \$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$h;:.                   !!!!!!!!
       !!!!!!!!!!'     \$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$h;.                   !!!!!!!
       !!!!!!!!!'     <\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$                   !!!!!!!
       !!!!!!!!'      `\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$F                   `!!!!!!
       !!!!!!!!        c\$\$\$\$???\$\$\$\$\$\$\$P\"\"  \"\"\"??????\"                      !!!!!!
       !!!!!!!         `\"\" .,.. \"\$\$\$\$F    .,zcr                            !!!!!!
       !!!!!!!         .  dL    .?\$\$\$   .,cc,      .,z\$h.                  !!!!!!
       !!!!!!!!        <. \$\$c= <\$d\$\$\$   <\$\$\$\$=-=+\"\$\$\$\$\$\$\$                  !!!!!!
       !!!!!!!         d\$\$\$hcccd\$\$\$\$\$   d\$\$\$hcccd\$\$\$\$\$\$\$F                  `!!!!!
       !!!!!!         ,\$\$\$\$\$\$\$\$\$\$\$\$\$\$h d\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$                   `!!!!!
       !!!!!          `\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$<\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$'                    !!!!!
       !!!!!          `\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\"\$\$\$\$\$\$\$\$\$\$\$\$\$P>                     !!!!!
       !!!!!           ?\$\$\$\$\$\$\$\$\$\$\$\$??\$c`\$\$\$\$\$\$\$\$\$\$\$?>'                     `!!!!
       !!!!!           `?\$\$\$\$\$\$I7?\"\"    ,\$\$\$\$\$\$\$\$\$?>>'                       !!!!
       !!!!!.           <<?\$\$\$\$\$\$c.    ,d\$\$?\$\$\$\$\$F>>''                       `!!!
       !!!!!!            <i?\$P\"??\$\$r--\"?\"\"  ,\$\$\$\$h;>''                       `!!!
       !!!!!!             \$\$\$hccccccccc= cc\$\$\$\$\$\$\$>>'                         !!!
       !!!!!              `?\$\$\$\$\$\$F\"\"\"\"  `\"\$\$\$\$\$>>>''                         `!!
       !!!!!                \"?\$\$\$\$\$cccccc\$\$\$\$??>>>>'                           !!
       !!!!>                  \"\$\$\$\$\$\$\$\$\$\$\$\$\$F>>>>''                            `!
       !!!!!                    \"\$\$\$\$\$\$\$\$???>'''                                !
       !!!!!>                     `\"\"\"\"\"                                        `
       !!!!!!;                       .                                          `
       !!!!!!!                       ?h.
       !!!!!!!!                       \$\$c,
       !!!!!!!!>                      ?\$\$\$h.              .,c
       !!!!!!!!!                       \$\$\$\$\$\$\$\$\$hc,.,,cc\$\$\$\$\$
       !!!!!!!!!                  .,zcc\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$
       !!!!!!!!!               .z\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$
       !!!!!!!!!             ,d\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$          .
       !!!!!!!!!           ,d\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$         !!
       !!!!!!!!!         ,d\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$        ,!'
       !!!!!!!!>        c\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$.
       !!!!!!''       ,d\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$\$      allen mullen
       """

julia_main()
