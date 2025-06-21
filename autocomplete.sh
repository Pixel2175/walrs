#!/bin/bash

# walrs Auto-completion Installer
# Simple, single-file installer that detects shell using $SHELL

set -e

# Get shell from $SHELL variable
CURRENT_SHELL=$(basename "$SHELL")

# Create completion based on shell
case "$CURRENT_SHELL" in
    "fish")
        echo "Installing Fish completion..."
        
        # Create fish completions directory
        mkdir -p ~/.config/fish/completions
        
        # Write fish completion file
        cat > ~/.config/fish/completions/walrs.fish << 'EOF'
# walrs completion for fish shell

# Clear existing completions
complete -c walrs -e

# Basic flags
complete -c walrs -s i -l image -d "path to image or wallpapers directory" -r -F
complete -c walrs -s k -l backend -d "change the colors backend" -x -a "kmeans color_thief palette_extract all backends"
complete -c walrs -s r -l reload -d "reload without changing the wallpaper"
complete -c walrs -s R -l reload-no -d "reload with changing the wallpaper"  
complete -c walrs -s t -l theme -d "use external theme file" -r -F
complete -c walrs -s g -l generate -d "generate theme in themes folder" -x
complete -c walrs -s s -l saturation -d "saturation value -128 to 127" -x
complete -c walrs -s b -l brightness -d "brightness value -128 to 127" -x
complete -c walrs -s q -l quit -d "set quit mode (no output)"
complete -c walrs -s v -l version -d "show version"
complete -c walrs -l help -d "display help"
complete -c walrs -a help -d "display help"

# Saturation and brightness numeric completion
for i in (seq -128 127)
    complete -c walrs -s s -l saturation -x -a "$i"
    complete -c walrs -s b -l brightness -x -a "$i"
end
EOF
        
        echo -e "Fish completion installed to: ~/.config/fish/completions/walrs.fish"
        ;;
        
    "zsh")
        echo "Installing Zsh completion..."
        
        # Create zsh completions directory
        mkdir -p ~/.zsh/completions
        
        # Write zsh completion file
        cat > ~/.zsh/completions/_walrs << 'EOF'
#compdef walrs

_walrs() {
    local context state line
    typeset -A opt_args

    _arguments \
        '(-i --image)'{-i,--image}'[path to image or wallpapers directory]:image file:_files -g "*.{jpg,jpeg,png,bmp,gif,tiff,webp}(-.)"' \
        '(-k --backend)'{-k,--backend}'[change the colors backend]:backend:(kmeans color_thief palette_extract all backends)' \
        '(-r --reload)'{-r,--reload}'[reload without changing the wallpaper]' \
        '(-R --reload-no)'{-R,--reload-no}'[reload with changing the wallpaper]' \
        '(-t --theme)'{-t,--theme}'[use external theme file]:theme file:_files -g "*.json(-.)"' \
        '(-g --generate)'{-g,--generate}'[generate theme in themes folder]:theme name:' \
        '(-s --saturation)'{-s,--saturation}'[specify the saturation value -128 => 127]:saturation:({-128..127})' \
        '(-b --brightness)'{-b,--brightness}'[specify the brightness value -128 => 127]:brightness:({-128..127})' \
        '(-q --quit)'{-q,--quit}'[set quit mode (no output)]' \
        '(-v --version)'{-v,--version}'[version]' \
        '(--help)--help[display usage information]'
}

_walrs "$@"
EOF
        
        # Add to fpath in .zshrc if not already there
        if ! grep -q "fpath=(.*\.zsh/completions" ~/.zshrc 2>/dev/null; then
            echo 'fpath=(~/.zsh/completions $fpath)' >> ~/.zshrc
            echo -e "Added ~/.zsh/completions to fpath in ~/.zshrc"
        fi
        
        echo -e "Zsh completion installed to: ~/.zsh/completions/_walrs"
        echo -e "Restart zsh or run: source ~/.zshrc && compinit"
        ;;
        
    "bash")
        echo "Installing Bash completion..."
        
        # Try different locations for bash completion
        if [ -d "/usr/share/bash-completion/completions" ] && [ -w "/usr/share/bash-completion/completions" ]; then
            COMPLETION_DIR="/usr/share/bash-completion/completions"
        elif [ -d "/etc/bash_completion.d" ] && [ -w "/etc/bash_completion.d" ]; then
            COMPLETION_DIR="/etc/bash_completion.d"
        else
            # Use user directory
            COMPLETION_DIR="$HOME/.local/share/bash-completion/completions"
            mkdir -p "$COMPLETION_DIR"
        fi
        
        # Write bash completion file
        cat > "$COMPLETION_DIR/walrs" << 'EOF'
_walrs_completion() {
    shopt -s extglob
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    opts="-i --image -k --backend -r --reload -R --reload-no -t --theme -g --generate -s --saturation -b --brightness -q --quit -v --version --help help"

    case ${prev} in
        -i|--image)
            COMPREPLY=( $(compgen -f -X '!*.@(jpg|jpeg|png|bmp|gif|tiff|webp)' -- "${cur}") )
            COMPREPLY+=( $(compgen -d -- "${cur}") )
            return 0
            ;;
        -k|--backend)
            COMPREPLY=( $(compgen -W "kmeans color_thief palette_extract all backends" -- "${cur}") )
            return 0
            ;;
        -t|--theme)
            COMPREPLY=( $(compgen -f -X '!*.json' -- "${cur}") )
            return 0
            ;;
        -g|--generate)
            COMPREPLY=( $(compgen -W "" -- "${cur}") )
            return 0
            ;;
        -s|--saturation|-b|--brightness)
            COMPREPLY=( $(compgen -W "$(seq -128 127)" -- "${cur}") )
            return 0
            ;;
    esac

    COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
    return 0
}

complete -F _walrs_completion walrs
EOF
        

        echo "source ~/.local/share/bash-completion/completions/walrs" >> ~/.bashrc
        echo -e "Bash completion installed to: $COMPLETION_DIR/walrs$"
        ;;
        
    *)
        echo -e "Unsupported shell: $CURRENT_SHELL"
        echo -e "Supported shells: fish, zsh, bash"
        exit 1
        ;;
esac
