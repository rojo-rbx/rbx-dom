local CanonicalInstance = require(script.CanonicalInstance)

return {
	ReflectionDatabase = require(script.ReflectionDatabase)
	EncodedValue = require(script.EncodedValue),
	CanonicalProperty = require(script.CanonicalProperty),
	CanonicalInstance = CanonicalInstance,
	PropertySelection = CanonicalInstance.PropertySelection,
}