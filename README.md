# medical-office

## Dev

`brew install rbenv ruby-build`
`echo 'eval "$(rbenv init -)"' >> ~/.zshrc`
`zsh`
`rbenv install 2.6.10`
`rbenv global 2.6.10`

`gem install bundler -v '~> 1.17'`  
`bundle config set --local bundler 1.17.3`  
`bundle _1.17.3_ install`

`bundle exec rake assets:precompile RAILS_ENV=development`
`bundle exec rails server`

Go to <http://localhost:3000>
