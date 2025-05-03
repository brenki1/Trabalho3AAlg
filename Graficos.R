library(ggplot2)

tExec <- read.csv("resultados.csv")

ggplot(data = tExec, aes(x = Quantidade))+
  #geom_line(aes(y = exponencial, color = "Exponencial O(k^n)"))+
  #geom_line(aes(y = aproximado))+
  geom_smooth(aes(y = Solução.aproximado), method = "loess", se = FALSE, color = "blue")+
  geom_smooth(aes(y = Solução.exponencial), method = "loess", se = FALSE, color = "red") +
  scale_y_continuous(name = "Quantidade de caixas usadas")+
  scale_x_continuous(name = "Quantidade de itens")+
  #labs(colour = "Algoritmos") +
  theme_bw()

