# bus factor estimation
Compute bus factor for most popular projects in a given language listed on GitHub.

# what is bus factor
The bus factor is a measurement of the risk resulting from information and capabilities not being shared among team members, derived from the phrase "in case they get hit by a bus". 
The "bus factor" is the minimum number of team members that have to suddenly disappear from a project before the project stalls due to lack of knowledgeable or competent personnel.

# how it works
Firstly, fetch most starred projects for a given programming language. Then, for each project, this program uses contributor statistics from the top 25 most active contributors and outputs data only if the most active developer is accountable for 75% or more of the total contributions.

# usage
This program uses Github API to fetch data. It is required from you to provide your GitHub personal access token via environment variable `BUSFACTOR_GITHUB_ACCESS_TOKEN`

```
export BUSFACTOR_GITHUB_ACCESS_TOKEN="YOUR_GITHUB_ACCESS_TOKEN"
```
```
./busfactor --language rust --project_count 50
project: 996.ICU                       user: 996icu                        percentage: 0.80
project: rustdesk                      user: rustdesk                      percentage: 0.80
project: Rocket                        user: SergioBenitez                 percentage: 0.86
project: exa                           user: ogham                         percentage: 0.85
project: ripgrep                       user: BurntSushi                    percentage: 0.89
project: appflowy                      user: AppFlowy-IO                   percentage: 0.81
project: iced                          user: iced-rs                       percentage: 0.89
project: sonic                         user: valeriansaliou                percentage: 0.92
project: delta                         user: dandavison                    percentage: 0.87
project: xsv                           user: BurntSushi                    percentage: 0.92
project: swc                           user: swc-project                   percentage: 0.78
project: pyxel                         user: kitao                         percentage: 0.98
project: navi                          user: denisidoro                    percentage: 0.78
project: hyper                         user: hyperium                      percentage: 0.79
project: book                          user: rust-lang                     percentage: 0.76
```
