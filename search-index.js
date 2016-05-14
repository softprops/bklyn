var searchIndex = {};
searchIndex["bklyn"] = {"doc":"Bklyn is a query interface for the kubernetes cluster\nmetric service [heapster](https://github.com/kubernetes/heapster).","items":[[3,"Value","bklyn","",null,null],[12,"timestamp","","",0,null],[12,"value","","",0,null],[3,"MetricCollection","","",null,null],[12,"metrics","","",1,null],[3,"Summary","","",null,null],[12,"name","","",2,null],[12,"cpu_usage","","",2,null],[12,"mem_usage","","",2,null],[3,"Aggregate","","",null,null],[12,"average","","",3,null],[12,"percentile","","",3,null],[12,"max","","",3,null],[3,"Aggregates","","",null,null],[12,"minute","","",4,null],[12,"hour","","",4,null],[12,"day","","",4,null],[3,"Stats","","",null,null],[12,"uptime","","",5,null],[12,"stats","","",5,null],[3,"MetricOptions","","query options for fetching metric values",null,null],[3,"MetricOptionsBuilder","","",null,null],[3,"Metrics","","metric interface",null,null],[3,"Node","","A node is essentially a host within a cluster",null,null],[3,"FreeContainer","","Metrics associated with a container not bound to a specific pod",null,null],[3,"NamespacePod","","Metrics associated with a pod within a given namespace",null,null],[3,"NamespacePodContainer","","Metrics associated with a container, within a pod, within a namespace",null,null],[3,"Namespace","","Metrics within a cluster namespace",null,null],[3,"Cluster","","Metrics associated with a kubernetes cluster",null,null],[3,"Heapster","","Central interface for communicating kubernetes heapster service",null,null],[4,"Error","","enumerated types of client errors",null,null],[13,"Codec","","",6,null],[13,"Http","","",6,null],[13,"IO","","",6,null],[13,"Parse","","",6,null],[13,"Fault","","",6,null],[12,"code","bklyn::Error","",6,null],[4,"Credentials","bklyn","Credentials used for authenticating with kubernetes cluster",null,null],[13,"Basic","","",7,null],[11,"fmt","","",0,{"inputs":[{"name":"value"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",1,{"inputs":[{"name":"metriccollection"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",2,{"inputs":[{"name":"summary"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",3,{"inputs":[{"name":"aggregate"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",4,{"inputs":[{"name":"aggregates"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",5,{"inputs":[{"name":"stats"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",6,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",6,{"inputs":[{"name":"error"}],"output":{"name":"str"}}],[11,"cause","","",6,{"inputs":[{"name":"error"}],"output":{"name":"option"}}],[11,"fmt","","",6,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",6,{"inputs":[{"name":"serdeerror"}],"output":{"name":"error"}}],[11,"from","","",6,{"inputs":[{"name":"httperror"}],"output":{"name":"error"}}],[11,"from","","",6,{"inputs":[{"name":"ioerror"}],"output":{"name":"error"}}],[6,"Result","","Result type for bklyn query operations",null,null],[11,"default","","",8,{"inputs":[],"output":{"name":"metricoptions"}}],[11,"builder","","",8,{"inputs":[],"output":{"name":"metricoptionsbuilder"}}],[11,"serialize","","",8,{"inputs":[{"name":"metricoptions"}],"output":{"name":"option"}}],[11,"default","","",9,{"inputs":[],"output":{"name":"metricoptionsbuilder"}}],[11,"start","","start timestamp, in RFC3339 format",9,{"inputs":[{"name":"metricoptionsbuilder"},{"name":"s"}],"output":{"name":"metricoptionsbuilder"}}],[11,"end","","end timestamp, in RFC3339 format",9,{"inputs":[{"name":"metricoptionsbuilder"},{"name":"e"}],"output":{"name":"metricoptionsbuilder"}}],[11,"build","","",9,{"inputs":[{"name":"metricoptionsbuilder"}],"output":{"name":"metricoptions"}}],[11,"names","","query availble metric names",10,{"inputs":[{"name":"metrics"}],"output":{"name":"result"}}],[11,"values","","query recorded metric values",10,{"inputs":[{"name":"metrics"},{"name":"m"},{"name":"metricoptions"}],"output":{"name":"result"}}],[11,"metrics","","",11,{"inputs":[{"name":"node"}],"output":{"name":"metrics"}}],[11,"stats","","",11,{"inputs":[{"name":"node"}],"output":{"name":"result"}}],[11,"pods","","",11,{"inputs":[{"name":"node"}],"output":{"name":"result"}}],[11,"freecontainers","","",11,{"inputs":[{"name":"node"}],"output":{"name":"result"}}],[11,"freecontainer","","",11,{"inputs":[{"name":"node"},{"name":"c"}],"output":{"name":"freecontainer"}}],[11,"metrics","","list metric names defined for this node container",12,{"inputs":[{"name":"freecontainer"}],"output":{"name":"metrics"}}],[11,"stats","","",12,{"inputs":[{"name":"freecontainer"}],"output":{"name":"result"}}],[11,"metrics","","",13,{"inputs":[{"name":"namespacepod"}],"output":{"name":"metrics"}}],[11,"stats","","",13,{"inputs":[{"name":"namespacepod"}],"output":{"name":"result"}}],[11,"containers","","",13,{"inputs":[{"name":"namespacepod"}],"output":{"name":"result"}}],[11,"container","","",13,{"inputs":[{"name":"namespacepod"},{"name":"c"}],"output":{"name":"namespacepodcontainer"}}],[11,"metrics","","",14,{"inputs":[{"name":"namespacepodcontainer"}],"output":{"name":"metrics"}}],[11,"stats","","",14,{"inputs":[{"name":"namespacepodcontainer"}],"output":{"name":"result"}}],[11,"metrics","","list metric names defined for this namespace",15,{"inputs":[{"name":"namespace"}],"output":{"name":"metrics"}}],[11,"stats","","",15,{"inputs":[{"name":"namespace"}],"output":{"name":"result"}}],[11,"pods","","",15,{"inputs":[{"name":"namespace"}],"output":{"name":"result"}}],[11,"pod","","",15,{"inputs":[{"name":"namespace"},{"name":"n"}],"output":{"name":"namespacepod"}}],[11,"metrics","","list metric names defined for this cluster",16,{"inputs":[{"name":"cluster"}],"output":{"name":"metrics"}}],[11,"stats","","query aggregate stats for cluster",16,{"inputs":[{"name":"cluster"}],"output":{"name":"result"}}],[11,"nodes","","list cluster nodes",16,{"inputs":[{"name":"cluster"}],"output":{"name":"result"}}],[11,"node","","return a query interface for a cluster node",16,{"inputs":[{"name":"cluster"},{"name":"n"}],"output":{"name":"node"}}],[11,"namespaces","","list cluster namespaces",16,{"inputs":[{"name":"cluster"}],"output":{"name":"result"}}],[11,"namespace","","return a query interface for a cluster namespace",16,{"inputs":[{"name":"cluster"},{"name":"n"}],"output":{"name":"namespace"}}],[11,"new","","create a new heapster instance",17,{"inputs":[{"name":"b"},{"name":"client"},{"name":"credentials"}],"output":{"name":"heapster"}}],[11,"cluster","","return a query interface for entire cluster",17,{"inputs":[{"name":"heapster"}],"output":{"name":"cluster"}}]],"paths":[[3,"Value"],[3,"MetricCollection"],[3,"Summary"],[3,"Aggregate"],[3,"Aggregates"],[3,"Stats"],[4,"Error"],[4,"Credentials"],[3,"MetricOptions"],[3,"MetricOptionsBuilder"],[3,"Metrics"],[3,"Node"],[3,"FreeContainer"],[3,"NamespacePod"],[3,"NamespacePodContainer"],[3,"Namespace"],[3,"Cluster"],[3,"Heapster"]]};
initSearch(searchIndex);