library(ggplot2)

tExec <- read.csv("resultados.csv")

ggplot(data = tExec, aes(x = Quantidade.trabalhos))+
  #geom_line(aes(y = exponencial, color = "Exponencial O(k^n)"))+
  #geom_line(aes(y = aproximado))+
  geom_smooth(aes(y = exponencial), method = "loess", se = FALSE, color = "red") +
  scale_y_time(name = "Tempo de execução (s)")+
  scale_x_continuous(name = "Quantidade de itens")+
  #labs(colour = "Algoritmos") +
  theme_bw()

